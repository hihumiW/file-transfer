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

// get_app_snapshot 是前端启动和刷新 UI 时最常调用的 command。
// 它不修改状态，只把 Rust 当前掌握的设备、服务、任务和配置整理成一个 AppSnapshot。
#[tauri::command]
pub fn get_app_snapshot(state: State<'_, Arc<AppState>>) -> Result<AppSnapshot, String> {
    snapshot(&state)
}

// save_device_name 从前端接收用户输入的设备名，校验后写入本地配置。
// 返回新的快照，是为了让前端保存成功后立刻刷新设备信息。
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

// select_display_ip 只改变 UI 展示和复制用的推荐地址。
// 本地 HTTP 服务仍然监听 0.0.0.0，不会因为选择某个 IP 而缩小监听范围。
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

// choose_save_dir 打开系统目录选择器，并把用户选择的目录保存到配置中。
// rfd 是跨平台文件对话框库；用户取消选择时返回当前快照，不视为错误。
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

// open_save_dir 用系统文件管理器打开当前保存目录。
// 打开前先 ensure_save_dir，保证目录不存在时会自动创建。
#[tauri::command]
pub fn open_save_dir(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let save_dir = state.ensure_save_dir()?;
    open_path(save_dir)
}

// choose_files 打开系统文件选择器，返回前端可展示和发送的 LocalFile 列表。
// v0.1 允许多选文件，但目录会在发送前被拒绝。
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

// describe_paths 用于处理拖拽到窗口里的路径。
// 前端只能拿到路径字符串，Rust 负责补齐文件名、大小、是否目录等元数据。
#[tauri::command]
pub fn describe_paths(paths: Vec<String>) -> Result<Vec<LocalFile>, String> {
    paths
        .into_iter()
        .map(PathBuf::from)
        .map(local_file_from_path)
        .collect::<Result<Vec<_>, _>>()
}

// normalize_target_address 暴露给前端做输入规范化。
// 例如 192.168.1.23 会被转换成 http://192.168.1.23:7788。
#[tauri::command]
pub fn normalize_target_address(raw: String) -> Result<String, String> {
    normalize_address(&raw).map(|parsed| parsed.address)
}

// test_target_connection 是“测试连接”按钮的核心逻辑。
// 它会请求目标设备 /api/device，校验协议版本和接收能力，成功后写入最近连接。
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

    // reqwest 是 Rust 端 HTTP 客户端；这里设置较短超时，避免 UI 长时间卡在连接测试。
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

    // TargetConnection 会返回给前端作为“当前发送目标”。
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

// delete_recent_device 按 address 删除一条最近连接记录。
// 删除后返回快照，前端无需手动维护本地列表副本。
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

// send_files 是发送文件的总入口，覆盖 v0.1 发送端完整流程：
// 校验文件 -> 提交传输请求 -> 等待接收方确认 -> 顺序上传文件 -> 返回最新快照。
#[tauri::command]
pub async fn send_files(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    target_address: String,
    files: Vec<LocalFile>,
) -> Result<AppSnapshot, String> {
    println!("into send_files!");
    if files.is_empty() {
        return Err("请选择要发送的文件".to_string());
    }
    if files.iter().any(|file| file.is_dir) {
        return Err("v0.1 暂不支持文件夹传输".to_string());
    }

    // 目标地址每次发送前重新规范化，避免前端传入未补协议或端口的字符串。
    let parsed = normalize_address(&target_address)?;
    println!("parsed {:?}", parsed);

    // 先克隆配置再释放锁，避免后续调用 state.save_dir() 时重复申请同一个 Mutex。
    let config = state
        .config
        .lock()
        .map_err(|_| "配置锁已损坏".to_string())?
        .clone();
    // save_dir 内部也会读取配置锁；此时上面的锁已经释放，不会出现自锁死等。
    let save_dir = state.save_dir()?;
    // local_device 会写入发送请求，让接收方确认弹窗能展示真实发送方设备名。
    let local_device = build_device_info(&config, &save_dir);
    println!("local_device {:?}", local_device);
    // 发送请求只包含文件元数据；真正的文件内容会在接收方 accepted 后再上传。
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
    println!("transfer_files : {:?}", transfer_files);
    let total_bytes = transfer_files.iter().map(|file| file.size).sum::<u64>();

    let client = Client::builder()
        // 大文件上传会让单个 HTTP 请求持续较久，不能沿用连接测试的短超时。
        .timeout(Duration::from_secs(60 * 60))
        .build()
        .map_err(|err| format!("创建 HTTP 客户端失败: {err}"))?;

    let request = TransferRequest {
        sender_device_id: Some(local_device.device_id.clone()),
        sender_device_name: local_device.device_name.clone(),
        files: transfer_files.clone(),
        total_bytes,
    };
    println!("before request {:?}", request);
    // 第一步：向接收端提交传输请求，让接收端创建 pending 任务并弹出确认 UI。
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
    // 发送端也创建一条本地任务，用同一个 transfer_id 关联后续进度和状态。
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

    // 第二步：轮询接收端状态。只有对方 accepted 后才进入真正上传。
    wait_until_accepted(&client, &parsed.address, &transfer_id, &state).await?;
    // 第三步：按文件列表顺序逐个流式上传，符合 v0.1 不并发上传的设计。
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

// respond_transfer 是接收方在确认弹窗点击“接收/拒绝”后调用的 command。
// 它只修改本机 Rust 状态；发送方会通过轮询 /api/transfer/status 得知结果。
#[tauri::command]
pub fn respond_transfer(
    state: State<'_, Arc<AppState>>,
    accept: bool,
    overwrite: bool,
) -> Result<AppSnapshot, String> {
    let (transfer_id, next_status) = {
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
            (item.transfer_id.clone(), TransferStatus::Accepted)
        } else {
            item.status = TransferStatus::Rejected;
            (item.transfer_id.clone(), TransferStatus::Rejected)
        }
    };

    // set_task_status 会再次读取 pending_transfer；必须在上面的锁释放后再调用，避免自锁死。
    set_task_status(&state, &transfer_id, next_status, None);
    snapshot(&state)
}

// clear_completed_tasks 只清理终态任务，不影响 pending/accepted/uploading 中的活跃任务。
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

// snapshot 把多个 Mutex 保护的状态读出来，组装成前端需要的一份 AppSnapshot。
// 读取时尽量短时间持锁，避免阻塞 HTTP handler 或其他 command 修改状态。
fn snapshot(state: &Arc<AppState>) -> Result<AppSnapshot, String> {
    let config = state
        .config
        .lock()
        .map_err(|_| "配置锁已损坏".to_string())?
        .clone();
    let save_dir = state.save_dir()?;
    let save_dir_available = ensure_writable_dir(&save_dir).is_ok();
    let addresses = list_network_addresses();
    // 如果用户手动选过 IP 且它仍在候选列表中，就优先使用；否则回退到推荐地址。
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
    // display_address 是给用户复制的地址，不能展示 0.0.0.0。
    // 如果没有可用局域网 IP，就用 127.0.0.1 作为本机调试兜底。
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

// ParsedAddress 是 normalize_address 的内部返回值。
// 对外展示用 address，最近连接和 UI 还需要拆开的 ip 与 port。
#[derive(Debug)]
struct ParsedAddress {
    address: String,
    ip: String,
    port: u16,
}

// normalize_address 实现 PRD 中的地址补全和校验规则。
// v0.1 明确只支持 HTTP + IPv4，因此这里会拒绝 https 和域名。
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

// map_connection_error 把 reqwest 的底层错误翻译成更适合用户理解的中文提示。
// 这里区分超时和连接失败，是为了分别提示“确认同局域网/应用已启动”和“检查端口/防火墙”。
fn map_connection_error(err: &reqwest::Error) -> String {
    if err.is_timeout() {
        "连接超时，请确认对方设备已启动应用并处于同一局域网".to_string()
    } else if err.is_connect() {
        "无法连接到该地址，请检查端口或防火墙".to_string()
    } else {
        format!("连接失败: {err}")
    }
}

// local_file_from_path 从文件系统读取路径元数据。
// 这一步只读取文件信息，不读取文件内容，因此对大文件也很轻量。
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

// wait_until_accepted 是发送端等待接收方确认的轮询循环。
// v0.1 不使用 WebSocket/SSE，所以每秒请求一次接收端状态接口。
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
                // 接收方已确认，发送端本地任务进入 Uploading，随后开始上传文件内容。
                set_task_status(state, transfer_id, TransferStatus::Uploading, None);
                return Ok(());
            }
            TransferStatus::Rejected => {
                // 接收方拒绝后，发送端任务进入 Rejected，并把错误返回给前端提示。
                set_task_status(state, transfer_id, TransferStatus::Rejected, None);
                return Err("对方已拒绝接收".to_string());
            }
            TransferStatus::Failed => {
                // 如果接收方在确认阶段失败，发送端同步失败状态和对方返回的 message。
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

// upload_files 按顺序上传多个文件。
// 每个文件都会被转换成异步字节流，reqwest 边读边发，避免把完整文件读入内存。
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
        // tokio::fs::File 是异步文件句柄，适合和 reqwest/axum 这类异步网络库一起使用。
        let source = File::open(&file.path)
            .await
            .map_err(|err| format!("文件不可读取 {}: {err}", file.name))?;
        // AtomicU64 用来在 stream 闭包里记录当前文件已发送字节数。
        // 这里使用 Arc，是因为闭包需要拥有一份可跨异步边界共享的计数器。
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
            // transferred_before_file + sent 得到整个任务维度的已发送字节数。
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

        // 这里把 transferId 和 fileIndex 放在 query 中，让接收端知道这个 body 属于哪个任务和第几个文件。
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

        // 单个文件请求完成后，再把任务总进度校准到该文件大小，避免 chunk 事件丢失导致进度不满。
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

// update_task_progress 只更新内存任务列表。
// 前端看到进度主要依赖 transfer-progress 事件，后续 refresh 时也能从快照中读到一致状态。
fn update_task_progress(state: &AppState, transfer_id: &str, transferred: u64, total: u64) {
    if let Ok(mut tasks) = state.tasks.lock() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == transfer_id) {
            task.transferred_bytes = transferred.min(total);
            task.status = TransferStatus::Uploading;
        }
    }
}

// percent 使用 saturating_mul 避免极大文件大小相乘时整数溢出。
// total 为 0 时按 100% 处理，兼容空文件传输。
fn percent(done: u64, total: u64) -> u8 {
    if total == 0 {
        100
    } else {
        ((done.saturating_mul(100) / total).min(100)) as u8
    }
}

// open_path 根据不同操作系统调用系统文件管理器打开目录。
// 这里使用 Command::spawn 启动外部程序，Rust 进程不会等待文件管理器关闭。
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
