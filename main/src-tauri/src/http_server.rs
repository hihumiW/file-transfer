use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::{connect_info::ConnectInfo, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::StreamExt;
use serde::Deserialize;
use tauri::{AppHandle, Emitter};
use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener};
use uuid::Uuid;

use crate::{
    device::{build_device_info, now_millis},
    models::{
        IncomingTransferEvent, PendingTransfer, ProgressEvent, TransferDirection, TransferFile,
        TransferRequest, TransferRequestResponse, TransferStatus, TransferStatusResponse,
        TransferTask, DEFAULT_PORT, MAX_PORT,
    },
    transfer::AppState,
};

const PROGRESS_EMIT_STEP_BYTES: u64 = 1024 * 1024;

// HttpContext 是 axum handler 的共享上下文。
// state 用来读写任务和配置，app 用来向本机 React 前端发送 Tauri event。
#[derive(Clone)]
struct HttpContext {
    state: Arc<AppState>,
    app: AppHandle,
}

// start_local_service 启动本机 HTTP 接收服务。
// 它会从 DEFAULT_PORT 递增尝试到 MAX_PORT，成功后把实际端口写入 AppState.service。
pub async fn start_local_service(state: Arc<AppState>, app: AppHandle) -> Result<(), String> {
    let mut last_error = None;

    for port in DEFAULT_PORT..=MAX_PORT {
        // 监听 0.0.0.0 表示接收所有网卡上的连接；UI 展示的具体 IP 由 device.rs 负责推荐。
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match TcpListener::bind(addr).await {
            Ok(listener) => {
                state.set_service_running(port)?;
                // Router 把 HTTP 路径映射到 handler。
                // with_state 会把 HttpContext 克隆进每个请求，避免使用全局静态变量。
                let router = Router::new()
                    .route("/api/device", get(api_device))
                    .route("/api/transfer/request", post(api_transfer_request))
                    .route(
                        "/api/transfer/status/{transfer_id}",
                        get(api_transfer_status),
                    )
                    .route("/api/transfer/upload", post(api_transfer_upload))
                    .with_state(HttpContext { state, app });

                // axum::serve 会一直运行，直到服务异常退出或应用进程结束。
                // 使用 ConnectInfo 把远端 socket 地址交给 handler，接收确认弹窗可以展示发送方 IP。
                axum::serve(
                    listener,
                    router.into_make_service_with_connect_info::<SocketAddr>(),
                )
                    .await
                    .map_err(|err| format!("本地接收服务异常退出: {err}"))?;
                return Ok(());
            }
            Err(err) => {
                last_error = Some(err.to_string());
            }
        }
    }

    let message = format!(
        "本地接收服务启动失败，请检查端口占用或防火墙设置。最后错误：{}",
        last_error.unwrap_or_else(|| "无可用端口".to_string())
    );
    state.set_service_error(message.clone())?;
    Err(message)
}

// api_device 对应 GET /api/device。
// 发送方用它做连接测试，并确认协议版本、设备名和当前是否允许接收。
async fn api_device(State(ctx): State<HttpContext>) -> impl IntoResponse {
    let config = ctx.state.config.lock().map(|guard| guard.clone());
    let save_dir = ctx.state.save_dir();

    match (config, save_dir) {
        (Ok(config), Ok(save_dir)) => Json(build_device_info(&config, &save_dir)).into_response(),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(build_device_info(&Default::default(), &Default::default())),
        )
            .into_response(),
    }
}

// api_transfer_request 对应 POST /api/transfer/request。
// 它只接收文件元数据，不接收文件内容；接收方确认后，发送方才会调用 upload 接口。
async fn api_transfer_request(
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    State(ctx): State<HttpContext>,
    Json(request): Json<TransferRequest>,
) -> impl IntoResponse {
    // 第一步只短暂读取接收门闩，判断当前是否已有活跃任务。
    let receiver_busy = match ctx.state.pending_transfer.lock() {
        Ok(guard) => guard
            .as_ref()
            .map(|pending| {
                matches!(
                    pending.status,
                    TransferStatus::Pending | TransferStatus::Accepted | TransferStatus::Uploading
                )
            })
            .unwrap_or(false),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransferRequestResponse {
                    ok: false,
                    transfer_id: None,
                    status: None,
                    error_code: Some("state_lock_failed".to_string()),
                    message: Some("接收方状态不可用".to_string()),
                }),
            )
                .into_response()
        }
    };

    // 第二步在不持有 pending_transfer 锁的情况下返回忙碌结果。
    if receiver_busy {
        return Json(TransferRequestResponse {
            ok: false,
            transfer_id: None,
            status: None,
            error_code: Some("receiver_busy".to_string()),
            message: Some("接收方当前正在处理其他传输任务".to_string()),
        })
        .into_response();
    }

    // 接收请求阶段就检查保存目录，能在上传前失败，避免浪费网络和磁盘 I/O。
    if ctx.state.ensure_save_dir().is_err() {
        return Json(TransferRequestResponse {
            ok: false,
            transfer_id: None,
            status: None,
            error_code: Some("save_dir_unavailable".to_string()),
            message: Some("接收方保存目录不可用".to_string()),
        })
        .into_response();
    }

    // transfer_id 由接收方生成，接收方是任务状态的权威来源。
    let transfer_id = Uuid::new_v4().to_string();
    let save_dir = ctx.state.save_dir().unwrap_or_default();
    // 在确认前检查重名文件，让前端能展示“覆盖全部或取消”的选择。
    let duplicate_files = request
        .files
        .iter()
        .filter(|file| save_dir.join(&file.name).exists())
        .map(|file| file.name.clone())
        .collect::<Vec<_>>();
    // remote_addr 来自 TCP 连接本身，比请求体可信；UI 只展示 IP，避免临时端口造成干扰。
    let peer_address = remote_addr.ip().to_string();

    // PendingTransfer 用于驱动接收确认 UI。
    let pending = PendingTransfer {
        transfer_id: transfer_id.clone(),
        sender_device_id: request.sender_device_id.clone(),
        sender_device_name: request.sender_device_name.clone(),
        sender_address: peer_address.clone(),
        files: request.files.clone(),
        total_bytes: request.total_bytes,
        status: TransferStatus::Pending,
        overwrite_confirmed: duplicate_files.is_empty(),
        duplicate_files,
        created_at: now_millis(),
    };

    // TransferTask 用于右侧任务列表展示。
    // 它和 pending 共用同一个 transfer_id，后续状态变化会同时更新两边。
    let task = TransferTask {
        id: transfer_id.clone(),
        direction: TransferDirection::Receive,
        peer_device_id: request.sender_device_id,
        peer_device_name: request.sender_device_name,
        peer_address: Some(peer_address),
        files: request.files,
        total_bytes: request.total_bytes,
        transferred_bytes: 0,
        status: TransferStatus::Pending,
        message: None,
        created_at: pending.created_at,
    };

    // 最后再写入 pending_transfer，避免在持锁期间做目录检查、重名扫描和事件发送。
    if let Ok(mut pending_guard) = ctx.state.pending_transfer.lock() {
        if pending_guard
            .as_ref()
            .map(|pending| {
                matches!(
                    pending.status,
                    TransferStatus::Pending | TransferStatus::Accepted | TransferStatus::Uploading
                )
            })
            .unwrap_or(false)
        {
            return Json(TransferRequestResponse {
                ok: false,
                transfer_id: None,
                status: None,
                error_code: Some("receiver_busy".to_string()),
                message: Some("接收方当前正在处理其他传输任务".to_string()),
            })
            .into_response();
        }
        *pending_guard = Some(pending.clone());
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TransferRequestResponse {
                ok: false,
                transfer_id: None,
                status: None,
                error_code: Some("state_lock_failed".to_string()),
                message: Some("接收方状态不可用".to_string()),
            }),
        )
            .into_response();
    }

    if let Ok(mut tasks) = ctx.state.tasks.lock() {
        tasks.push(task);
    }

    // 通过 Tauri event 通知本机 React：有新的接收请求，需要展示确认弹窗。
    let _ = ctx.app.emit(
        "incoming-transfer",
        IncomingTransferEvent { transfer: pending },
    );

    Json(TransferRequestResponse {
        ok: true,
        transfer_id: Some(transfer_id),
        status: Some(TransferStatus::Pending),
        error_code: None,
        message: None,
    })
    .into_response()
}

// api_transfer_status 对应 GET /api/transfer/status/:transferId。
// 发送方会每秒轮询它，直到看到 accepted/rejected/failed 等终态或可上传状态。
async fn api_transfer_status(
    State(ctx): State<HttpContext>,
    Path(transfer_id): Path<String>,
) -> impl IntoResponse {
    // 状态优先从 pending_transfer 读取，因为它代表当前接收请求的最新确认状态。
    let status = ctx
        .state
        .pending_transfer
        .lock()
        .ok()
        .and_then(|guard| {
            guard
                .as_ref()
                .filter(|item| item.transfer_id == transfer_id)
                .cloned()
        })
        .map(|pending| pending.status)
        .or_else(|| {
            ctx.state
                .tasks
                .lock()
                .ok()
                .and_then(|tasks| tasks.iter().find(|task| task.id == transfer_id).cloned())
                .map(|task| task.status)
        });

    // 未找到 transfer_id 时返回 404，让发送方能把任务标记为失败。
    match status {
        Some(status) => Json(TransferStatusResponse {
            ok: true,
            transfer_id,
            status,
            message: None,
        })
        .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(TransferStatusResponse {
                ok: false,
                transfer_id,
                status: TransferStatus::Failed,
                message: Some("未找到传输任务".to_string()),
            }),
        )
            .into_response(),
    }
}

// UploadQuery 对应 /api/transfer/upload?transferId=...&fileIndex=...
// axum 的 Query extractor 会按 camelCase 解析前端/发送端传来的查询参数。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadQuery {
    transfer_id: String,
    file_index: usize,
}

// api_transfer_upload 对应 POST /api/transfer/upload。
// 请求体 body 是文件内容的异步流，handler 会边收边写入保存目录。
async fn api_transfer_upload(
    State(ctx): State<HttpContext>,
    Query(query): Query<UploadQuery>,
    body: Body,
) -> impl IntoResponse {
    // 上传必须关联到一个已存在的 pending 任务，否则接收端不知道该保存到哪个任务。
    let pending = match ctx.state.pending_transfer.lock() {
        Ok(guard) => guard
            .as_ref()
            .filter(|item| item.transfer_id == query.transfer_id)
            .cloned(),
        Err(_) => None,
    };

    let Some(pending) = pending else {
        return (StatusCode::NOT_FOUND, "transfer not found").into_response();
    };

    // 只有用户确认接收后才允许上传。
    // 这样能保证“发送请求”和“实际上传”这两个阶段严格分离。
    if !matches!(
        pending.status,
        TransferStatus::Accepted | TransferStatus::Uploading
    ) {
        return (StatusCode::CONFLICT, "transfer is not accepted").into_response();
    }

    // file_index 由发送端按照待发送列表顺序传入，用来取对应文件名和大小。
    let Some(file_meta) = pending.files.get(query.file_index).cloned() else {
        return (StatusCode::BAD_REQUEST, "file index out of range").into_response();
    };

    println!(
        "receive upload start transfer={} file_index={} name={} size={}",
        pending.transfer_id, query.file_index, file_meta.name, file_meta.size
    );
    // 真正的保存逻辑放到 save_upload_stream，handler 只负责把成功/失败转成 HTTP 响应。
    match save_upload_stream(
        &ctx,
        &pending.transfer_id,
        query.file_index,
        &file_meta,
        body,
    )
    .await
    {
        Ok(()) => {
            println!(
                "receive upload saved transfer={} file_index={} name={}",
                pending.transfer_id, query.file_index, file_meta.name
            );
            // 单文件写入完成后，累加任务总进度。
            mark_file_completed(&ctx.state, &pending.transfer_id, file_meta.size);
            // 如果这是最后一个文件，整个多文件任务就可以进入 Completed。
            if query.file_index + 1 == pending.files.len() {
                println!("receive task completed transfer={}", pending.transfer_id);
                set_task_status(
                    &ctx.state,
                    &pending.transfer_id,
                    TransferStatus::Completed,
                    None,
                );
                // 完成状态写入后再发一次事件，促使前端刷新到 completed，而不是停在最后一次 chunk 的 uploading。
                let _ = ctx.app.emit(
                    "transfer-progress",
                    ProgressEvent {
                        transfer_id: pending.transfer_id.clone(),
                        file_name: file_meta.name.clone(),
                        file_index: query.file_index + 1,
                        file_total: pending.files.len(),
                        transferred_bytes: pending.total_bytes,
                        total_bytes: pending.total_bytes,
                        percent: 100,
                        status: Some(TransferStatus::Completed),
                    },
                );
            }
            StatusCode::OK.into_response()
        }
        Err(err) => {
            println!(
                "receive upload failed transfer={} file_index={} error={}",
                pending.transfer_id, query.file_index, err
            );
            // 任一文件保存失败都会让整个任务失败，符合 v0.1 的“全收或全失败”模型。
            set_task_status(
                &ctx.state,
                &pending.transfer_id,
                TransferStatus::Failed,
                Some(err.clone()),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
        }
    }
}

// save_upload_stream 执行接收端最关键的流式写盘逻辑。
// 它从 axum Body 中逐块读取字节，立即写入目标文件，并通过 Tauri event 汇报接收进度。
async fn save_upload_stream(
    ctx: &HttpContext,
    transfer_id: &str,
    file_index: usize,
    file_meta: &TransferFile,
    body: Body,
) -> Result<(), String> {
    // 每次上传前重新 ensure_save_dir，可以覆盖用户运行中修改目录或目录被删除的情况。
    let save_dir = ctx.state.ensure_save_dir()?;
    let target_path = save_dir.join(&file_meta.name);
    // File::create 会创建或截断目标文件。
    // 是否允许覆盖在 transfer/request 阶段已经由 duplicate_files + overwrite_confirmed 决定。
    let mut file = File::create(&target_path)
        .await
        .map_err(|err| format!("创建文件失败 {}: {err}", target_path.display()))?;

    let mut received = 0_u64;
    let mut last_emitted = 0_u64;
    let mut stream = body.into_data_stream();

    // axum 的 Body 是一个异步流。这里逐块写入文件，避免把大文件完整读进内存。
    // 每个 chunk 都是网络层到达的一段字节；真实大小由 hyper/axum 决定，我们只需要
    // 处理“成功拿到字节 -> 写入磁盘 -> 更新进度”这条链路。
    while received < file_meta.size {
        let Some(chunk) = stream.next().await else {
            return Err(format!(
                "接收文件不完整 {}: 已接收 {} 字节，期望 {} 字节",
                file_meta.name, received, file_meta.size
            ));
        };
        let chunk = chunk.map_err(|err| format!("读取上传数据失败: {err}"))?;

        // 发送端可能不会立即关闭 HTTP body；接收端按协商的文件大小收满即可结束本文件。
        let remaining = file_meta.size.saturating_sub(received) as usize;
        let bytes_to_write = remaining.min(chunk.len());
        file.write_all(&chunk[..bytes_to_write])
            .await
            .map_err(|err| format!("写入文件失败 {}: {err}", target_path.display()))?;
        received += bytes_to_write as u64;

        // 接收端进度按当前文件计算；任务总进度会在 mark_file_completed 中按文件累加。
        let percent = if file_meta.size == 0 {
            100
        } else {
            ((received.saturating_mul(100) / file_meta.size).min(100)) as u8
        };
        if received == file_meta.size || received.saturating_sub(last_emitted) >= PROGRESS_EMIT_STEP_BYTES {
            last_emitted = received;
            let _ = ctx.app.emit(
                "transfer-progress",
                ProgressEvent {
                    transfer_id: transfer_id.to_string(),
                    file_name: file_meta.name.clone(),
                    file_index: file_index + 1,
                    file_total: 1,
                    transferred_bytes: received,
                    total_bytes: file_meta.size,
                    percent,
                    status: Some(TransferStatus::Uploading),
                },
            );
        }
    }

    if file_meta.size == 0 {
        let _ = ctx.app.emit(
            "transfer-progress",
            ProgressEvent {
                transfer_id: transfer_id.to_string(),
                file_name: file_meta.name.clone(),
                file_index: file_index + 1,
                file_total: 1,
                transferred_bytes: 0,
                total_bytes: 0,
                percent: 100,
                status: Some(TransferStatus::Uploading),
            },
        );
    }

    println!(
        "receive stream complete transfer={} file_index={} name={} received={} expected={}",
        transfer_id, file_index, file_meta.name, received, file_meta.size
    );
    // flush 确保缓冲区内容尽量写入系统，及时暴露磁盘写入错误。
    file.flush()
        .await
        .map_err(|err| format!("刷新文件失败 {}: {err}", target_path.display()))?;
    println!(
        "receive stream flushed transfer={} file_index={} name={}",
        transfer_id, file_index, file_meta.name
    );
    Ok(())
}

// set_task_status 是跨模块共享的小工具，用同一个 transfer_id 同步任务列表和 pending_transfer。
// 它被发送端轮询逻辑、接收确认逻辑和上传保存逻辑共同调用。
pub fn set_task_status(
    state: &AppState,
    transfer_id: &str,
    status: TransferStatus,
    message: Option<String>,
) {
    if let Ok(mut tasks) = state.tasks.lock() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == transfer_id) {
            task.status = status.clone();
            task.message = message.clone();
        }
    }

    if let Ok(mut pending) = state.pending_transfer.lock() {
        if let Some(item) = pending
            .as_mut()
            .filter(|item| item.transfer_id == transfer_id)
        {
            item.status = status;
        }
    }
}

// mark_file_completed 在接收端每完成一个文件后累加总任务进度。
// 使用 saturating_add 可以避免异常大小导致整数溢出。
fn mark_file_completed(state: &AppState, transfer_id: &str, size: u64) {
    if let Ok(mut tasks) = state.tasks.lock() {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == transfer_id) {
            task.status = TransferStatus::Uploading;
            task.transferred_bytes = task
                .transferred_bytes
                .saturating_add(size)
                .min(task.total_bytes);
        }
    }
}
