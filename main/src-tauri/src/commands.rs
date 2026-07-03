use std::{
    path::PathBuf,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::{
    config::ensure_writable_dir,
    device::{
        build_device_info, list_network_addresses, now_millis, upsert_recent_device,
        validate_device_name,
    },
    http_server::set_task_status,
    models::{
        AppSnapshot, DeviceInfo, LocalFile, ProgressEvent, RecentDevice, TargetConnection,
        TransferDirection, TransferFile, TransferRequest, TransferRequestResponse, TransferStatus,
        TransferStatusResponse, TransferTask, DEFAULT_PORT, PROTOCOL_VERSION,
    },
    transfer::AppState,
};
use futures_util::TryStreamExt;
use reqwest::Client;
use tauri::{AppHandle, Emitter, State};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[tauri::command]
pub fn get_app_snapshot(state: State<'_, Arc<AppState>>) -> Result<AppSnapshot, String> {
    snapshot(&state)
}

#[tauri::command]
pub fn save_device_name(
    state: State<'_, Arc<AppState>>,
    device_name: String,
) -> Result<AppSnapshot, String> {
    let valid_name = validate_device_name(&device_name)?;
    {
        let mut config = state
            .config
            .lock()
            .map_err(|_| "配置锁已损坏".to_string())?;
        config.custom_device_name = Some(valid_name);
        config.save()?;
    }
    snapshot(&state)
}

#[tauri::command]
pub fn select_display_ip(
    state: State<'_, Arc<AppState>>,
    ip: String,
) -> Result<AppSnapshot, String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|_| "配置锁已损坏".to_string())?;
        config.selected_ip = Some(ip);
        config.save()?;
    }
    snapshot(&state)
}

#[tauri::command]
pub fn choose_save_dir(state: State<'_, Arc<AppState>>) -> Result<AppSnapshot, String> {
    let Some(folder) = rfd::FileDialog::new().pick_folder() else {
        return snapshot(&state);
    };
    ensure_writable_dir(&folder)?;
    {
        let mut config = state
            .config
            .lock()
            .map_err(|_| "配置锁已损坏".to_string())?;
        config.save_dir = Some(folder.to_string_lossy().into_owned());
        config.save()?;
    }
    snapshot(&state)
}

#[tauri::command]
pub fn open_save_dir(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let save_dir = state.ensure_save_dir()?;
    open_path(save_dir)
}

#[tauri::command]
pub fn choose_files() -> Result<Vec<LocalFile>, String> {
    let Some(files) = rfd::FileDialog::new().pick_files() else {
        return Ok(Vec::new());
    };

    files
        .into_iter()
        .map(local_file_from_path)
        .collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub fn describe_paths(paths: Vec<String>) -> Result<Vec<LocalFile>, String> {
    paths
        .into_iter()
        .map(PathBuf::from)
        .map(local_file_from_path)
        .collect::<Result<Vec<_>, _>>()
}

#[tauri::command]
pub fn normalize_target_address(raw: String) -> Result<String, String> {
    normalize_address(&raw).map(|parsed| parsed.address)
}

#[tauri::command]
pub async fn test_target_connection(
    state: State<'_, Arc<AppState>>,
    raw: String,
) -> Result<TargetConnection, String> {
    let parsed = normalize_address(&raw)?;
    let client = Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .map_err(|err| format!("创建 HTTP 客户端失败: {err}"))?;

    let response = client
        .get(format!("{}/api/device", parsed.address))
        .send()
        .await
        .map_err(|err| map_connection_error(&err))?;

    let device = response
        .json::<DeviceInfo>()
        .await
        .map_err(|_| "目标地址不是有效的局域网传输工具".to_string())?;

    if !device.ok {
        return Err("目标地址不是有效的局域网传输工具".to_string());
    }
    if device.protocol_version != PROTOCOL_VERSION {
        return Err("对方应用版本暂不兼容".to_string());
    }
    if !device.receive_enabled {
        return Err("对方设备当前不可接收文件".to_string());
    }

    let target = TargetConnection {
        device,
        address: parsed.address,
        ip: parsed.ip,
        port: parsed.port,
    };

    {
        let mut config = state
            .config
            .lock()
            .map_err(|_| "配置锁已损坏".to_string())?;
        upsert_recent_device(
            &mut config.recent_devices,
            RecentDevice {
                device_id: Some(target.device.device_id.clone()),
                device_name: target.device.device_name.clone(),
                address: target.address.clone(),
                ip: target.ip.clone(),
                port: target.port,
                last_connected_at: now_millis(),
                last_success_at: None,
            },
        );
        config.save()?;
    }

    Ok(target)
}

#[tauri::command]
pub fn delete_recent_device(
    state: State<'_, Arc<AppState>>,
    address: String,
) -> Result<AppSnapshot, String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|_| "配置锁已损坏".to_string())?;
        config.recent_devices.retain(|item| item.address != address);
        config.save()?;
    }
    snapshot(&state)
}

#[tauri::command]
pub async fn send_files(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    target_address: String,
    files: Vec<LocalFile>,
) -> Result<AppSnapshot, String> {
    if files.is_empty() {
        return Err("请选择要发送的文件".to_string());
    }
    if files.iter().any(|file| file.is_dir) {
        return Err("v0.1 暂不支持文件夹传输".to_string());
    }

    let parsed = normalize_address(&target_address)?;
    let local_device = {
        let config = state
            .config
            .lock()
            .map_err(|_| "配置锁已损坏".to_string())?;
        let save_dir = state.save_dir()?;
        build_device_info(&config, &save_dir)
    };

    let transfer_files = files
        .iter()
        .map(|file| {
            if !PathBuf::from(&file.path).is_file() {
                return Err(format!("文件不可用: {}", file.name));
            }
            Ok(TransferFile {
                name: file.name.clone(),
                size: file.size,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let total_bytes = transfer_files.iter().map(|file| file.size).sum::<u64>();

    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .map_err(|err| format!("创建 HTTP 客户端失败: {err}"))?;

    let request = TransferRequest {
        sender_device_id: Some(local_device.device_id.clone()),
        sender_device_name: local_device.device_name.clone(),
        files: transfer_files.clone(),
        total_bytes,
    };

    let response = client
        .post(format!("{}/api/transfer/request", parsed.address))
        .json(&request)
        .send()
        .await
        .map_err(|err| map_connection_error(&err))?
        .json::<TransferRequestResponse>()
        .await
        .map_err(|err| format!("解析传输请求响应失败: {err}"))?;

    if !response.ok {
        return Err(response
            .message
            .unwrap_or_else(|| "对方拒绝了传输请求".to_string()));
    }

    let transfer_id = response
        .transfer_id
        .ok_or("对方未返回传输任务 ID".to_string())?;
    let task = TransferTask {
        id: transfer_id.clone(),
        direction: TransferDirection::Send,
        peer_device_id: None,
        peer_device_name: parsed.address.clone(),
        peer_address: Some(parsed.address.clone()),
        files: transfer_files.clone(),
        total_bytes,
        transferred_bytes: 0,
        status: TransferStatus::Pending,
        message: None,
        created_at: now_millis(),
    };
    {
        let mut tasks = state.tasks.lock().map_err(|_| "任务锁已损坏".to_string())?;
        tasks.push(task);
    }

    wait_until_accepted(&client, &parsed.address, &transfer_id, &state).await?;
    upload_files(
        &client,
        &app,
        &state,
        &parsed.address,
        &transfer_id,
        &files,
        total_bytes,
    )
    .await?;

    snapshot(&state)
}

#[tauri::command]
pub fn respond_transfer(
    state: State<'_, Arc<AppState>>,
    accept: bool,
    overwrite: bool,
) -> Result<AppSnapshot, String> {
    let mut pending = state
        .pending_transfer
        .lock()
        .map_err(|_| "接收任务锁已损坏".to_string())?;
    let Some(item) = pending.as_mut() else {
        return Err("没有待处理的接收请求".to_string());
    };

    if accept {
        if !item.duplicate_files.is_empty() && !overwrite {
            return Err("存在重名文件，请确认是否覆盖".to_string());
        }
        item.overwrite_confirmed = true;
        item.status = TransferStatus::Accepted;
        set_task_status(&state, &item.transfer_id, TransferStatus::Accepted, None);
    } else {
        item.status = TransferStatus::Rejected;
        set_task_status(&state, &item.transfer_id, TransferStatus::Rejected, None);
    }
    drop(pending);
    snapshot(&state)
}

#[tauri::command]
pub fn clear_completed_tasks(state: State<'_, Arc<AppState>>) -> Result<AppSnapshot, String> {
    {
        let mut tasks = state.tasks.lock().map_err(|_| "任务锁已损坏".to_string())?;
        tasks.retain(|task| {
            !matches!(
                task.status,
                TransferStatus::Completed | TransferStatus::Failed | TransferStatus::Rejected
            )
        });
    }
    snapshot(&state)
}

fn snapshot(state: &Arc<AppState>) -> Result<AppSnapshot, String> {
    let config = state
        .config
        .lock()
        .map_err(|_| "配置锁已损坏".to_string())?
        .clone();
    let save_dir = state.save_dir()?;
    let save_dir_available = ensure_writable_dir(&save_dir).is_ok();
    let addresses = list_network_addresses();
    let selected_ip = addresses
        .iter()
        .find(|item| {
            config
                .selected_ip
                .as_deref()
                .is_some_and(|selected| selected == item.ip)
        })
        .or_else(|| addresses.iter().find(|item| item.recommended))
        .map(|item| item.ip.clone());
    let service = state
        .service
        .lock()
        .map_err(|_| "服务状态锁已损坏".to_string())?
        .clone();
    let display_address = selected_ip
        .as_ref()
        .map(|ip| format!("http://{}:{}", ip, service.port))
        .unwrap_or_else(|| format!("http://127.0.0.1:{}", service.port));

    Ok(AppSnapshot {
        device: build_device_info(&config, &save_dir),
        display_address,
        selected_ip,
        network_addresses: addresses,
        service,
        save_dir: save_dir.to_string_lossy().into_owned(),
        save_dir_available,
        recent_devices: config.recent_devices,
        tasks: state
            .tasks
            .lock()
            .map_err(|_| "任务锁已损坏".to_string())?
            .clone(),
        pending_transfer: state
            .pending_transfer
            .lock()
            .map_err(|_| "接收任务锁已损坏".to_string())?
            .clone(),
    })
}

#[derive(Debug)]
struct ParsedAddress {
    address: String,
    ip: String,
    port: u16,
}

fn normalize_address(raw: &str) -> Result<ParsedAddress, String> {
    let input = raw.trim();
    if input.is_empty() {
        return Err("地址格式不正确".to_string());
    }
    if input.starts_with("https://") {
        return Err("v0.1 仅支持 HTTP 地址".to_string());
    }

    let with_scheme = if input.starts_with("http://") {
        input.to_string()
    } else {
        format!("http://{input}")
    };

    let url = url::Url::parse(&with_scheme).map_err(|_| "地址格式不正确".to_string())?;
    if url.scheme() != "http" {
        return Err("v0.1 仅支持 HTTP 地址".to_string());
    }
    let host = url.host_str().ok_or("地址格式不正确".to_string())?;
    if host.parse::<std::net::Ipv4Addr>().is_err() {
        return Err("v0.1 暂仅支持 IPv4 地址".to_string());
    }
    let port = url.port().unwrap_or(DEFAULT_PORT);

    Ok(ParsedAddress {
        address: format!("http://{host}:{port}"),
        ip: host.to_string(),
        port,
    })
}

fn map_connection_error(err: &reqwest::Error) -> String {
    if err.is_timeout() {
        "连接超时，请确认对方设备已启动应用并处于同一局域网".to_string()
    } else if err.is_connect() {
        "无法连接到该地址，请检查端口或防火墙".to_string()
    } else {
        format!("连接失败: {err}")
    }
}

fn local_file_from_path(path: PathBuf) -> Result<LocalFile, String> {
    let metadata = path
        .metadata()
        .map_err(|err| format!("读取文件信息失败 {}: {err}", path.display()))?;
    Ok(LocalFile {
        name: path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned()),
        path: path.to_string_lossy().into_owned(),
        size: metadata.len(),
        is_dir: metadata.is_dir(),
    })
}

async fn wait_until_accepted(
    client: &Client,
    target: &str,
    transfer_id: &str,
    state: &Arc<AppState>,
) -> Result<(), String> {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let response = client
            .get(format!("{target}/api/transfer/status/{transfer_id}"))
            .send()
            .await
            .map_err(|err| map_connection_error(&err))?
            .json::<TransferStatusResponse>()
            .await
            .map_err(|err| format!("解析传输状态失败: {err}"))?;

        match response.status {
            TransferStatus::Accepted => {
                set_task_status(state, transfer_id, TransferStatus::Uploading, None);
                return Ok(());
            }
            TransferStatus::Rejected => {
                set_task_status(state, transfer_id, TransferStatus::Rejected, None);
                return Err("对方已拒绝接收".to_string());
            }
            TransferStatus::Failed => {
                set_task_status(
                    state,
                    transfer_id,
                    TransferStatus::Failed,
                    response.message.clone(),
                );
                return Err(response
                    .message
                    .unwrap_or_else(|| "对方传输任务失败".to_string()));
            }
            _ => {}
        }
    }
}

async fn upload_files(
    client: &Client,
    app: &AppHandle,
    state: &Arc<AppState>,
    target: &str,
    transfer_id: &str,
    files: &[LocalFile],
    total_bytes: u64,
) -> Result<(), String> {
    let mut transferred = 0_u64;

    for (index, file) in files.iter().enumerate() {
        let source = File::open(&file.path)
            .await
            .map_err(|err| format!("文件不可读取 {}: {err}", file.name))?;
        let sent_for_file = Arc::new(AtomicU64::new(0));
        let app_for_stream = app.clone();
        let state_for_stream = state.clone();
        let transfer_id_for_stream = transfer_id.to_string();
        let file_name_for_stream = file.name.clone();
        let file_size = file.size;
        let file_total = files.len();
        let file_number = index + 1;
        let transferred_before_file = transferred;

        // ReaderStream 会把 tokio::fs::File 变成 reqwest 可以消费的异步字节流。
        // map_ok 在每个 chunk 成功读取后执行，因此适合做发送端进度统计；这里不要在
        // 闭包里做耗时工作，只更新内存状态并发一个轻量事件。
        let stream = ReaderStream::new(source).map_ok(move |chunk| {
            let sent =
                sent_for_file.fetch_add(chunk.len() as u64, Ordering::SeqCst) + chunk.len() as u64;
            update_task_progress(
                &state_for_stream,
                &transfer_id_for_stream,
                transferred_before_file.saturating_add(sent),
                total_bytes,
            );
            let _ = app_for_stream.emit(
                "transfer-progress",
                ProgressEvent {
                    transfer_id: transfer_id_for_stream.clone(),
                    file_name: file_name_for_stream.clone(),
                    file_index: file_number,
                    file_total,
                    transferred_bytes: sent,
                    total_bytes: file_size,
                    percent: percent(sent, file_size),
                },
            );
            chunk
        });

        client
            .post(format!(
                "{target}/api/transfer/upload?transferId={transfer_id}&fileIndex={index}"
            ))
            .body(reqwest::Body::wrap_stream(stream))
            .send()
            .await
            .map_err(|err| map_connection_error(&err))?
            .error_for_status()
            .map_err(|err| format!("上传失败 {}: {err}", file.name))?;

        transferred = transferred.saturating_add(file.size).min(total_bytes);
        update_task_progress(state, transfer_id, transferred, total_bytes);
        let _ = app.emit(
            "transfer-progress",
            ProgressEvent {
                transfer_id: transfer_id.to_string(),
                file_name: file.name.clone(),
                file_index: index + 1,
                file_total: files.len(),
                transferred_bytes: transferred,
                total_bytes,
                percent: percent(transferred, total_bytes),
            },
        );
    }

    set_task_status(state, transfer_id, TransferStatus::Completed, None);
    Ok(())
}

fn update_task_progress(state: &AppState, transfer_id: &str, transferred: u64, total: u64) {
    if let Ok(mut tasks) = state.tasks.lock() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == transfer_id) {
            task.transferred_bytes = transferred.min(total);
            task.status = TransferStatus::Uploading;
        }
    }
}

fn percent(done: u64, total: u64) -> u8 {
    if total == 0 {
        100
    } else {
        ((done.saturating_mul(100) / total).min(100)) as u8
    }
}

fn open_path(path: PathBuf) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut cmd = Command::new("explorer");
        cmd.arg(path);
        cmd
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut cmd = Command::new("open");
        cmd.arg(path);
        cmd
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut cmd = Command::new("xdg-open");
        cmd.arg(path);
        cmd
    };

    command
        .spawn()
        .map(|_| ())
        .map_err(|err| format!("打开目录失败: {err}"))
}
