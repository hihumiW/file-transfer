use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, Query, State},
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

#[derive(Clone)]
struct HttpContext {
    state: Arc<AppState>,
    app: AppHandle,
}

pub async fn start_local_service(state: Arc<AppState>, app: AppHandle) -> Result<(), String> {
    let mut last_error = None;

    for port in DEFAULT_PORT..=MAX_PORT {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match TcpListener::bind(addr).await {
            Ok(listener) => {
                state.set_service_running(port)?;
                let router = Router::new()
                    .route("/api/device", get(api_device))
                    .route("/api/transfer/request", post(api_transfer_request))
                    .route(
                        "/api/transfer/status/{transfer_id}",
                        get(api_transfer_status),
                    )
                    .route("/api/transfer/upload", post(api_transfer_upload))
                    .with_state(HttpContext { state, app });

                axum::serve(listener, router)
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

async fn api_transfer_request(
    State(ctx): State<HttpContext>,
    Json(request): Json<TransferRequest>,
) -> impl IntoResponse {
    let mut pending_guard = match ctx.state.pending_transfer.lock() {
        Ok(guard) => guard,
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

    let transfer_id = Uuid::new_v4().to_string();
    let save_dir = ctx.state.save_dir().unwrap_or_default();
    let duplicate_files = request
        .files
        .iter()
        .filter(|file| save_dir.join(&file.name).exists())
        .map(|file| file.name.clone())
        .collect::<Vec<_>>();
    let peer_address = "unknown".to_string();

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

    *pending_guard = Some(pending.clone());
    if let Ok(mut tasks) = ctx.state.tasks.lock() {
        tasks.push(task);
    }

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

async fn api_transfer_status(
    State(ctx): State<HttpContext>,
    Path(transfer_id): Path<String>,
) -> impl IntoResponse {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadQuery {
    transfer_id: String,
    file_index: usize,
}

async fn api_transfer_upload(
    State(ctx): State<HttpContext>,
    Query(query): Query<UploadQuery>,
    body: Body,
) -> impl IntoResponse {
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

    if !matches!(
        pending.status,
        TransferStatus::Accepted | TransferStatus::Uploading
    ) {
        return (StatusCode::CONFLICT, "transfer is not accepted").into_response();
    }

    let Some(file_meta) = pending.files.get(query.file_index).cloned() else {
        return (StatusCode::BAD_REQUEST, "file index out of range").into_response();
    };

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
            mark_file_completed(&ctx.state, &pending.transfer_id, file_meta.size);
            if query.file_index + 1 == pending.files.len() {
                set_task_status(
                    &ctx.state,
                    &pending.transfer_id,
                    TransferStatus::Completed,
                    None,
                );
                if let Ok(mut guard) = ctx.state.pending_transfer.lock() {
                    if let Some(item) = guard.as_mut() {
                        item.status = TransferStatus::Completed;
                    }
                }
            }
            StatusCode::OK.into_response()
        }
        Err(err) => {
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

async fn save_upload_stream(
    ctx: &HttpContext,
    transfer_id: &str,
    file_index: usize,
    file_meta: &TransferFile,
    body: Body,
) -> Result<(), String> {
    let save_dir = ctx.state.ensure_save_dir()?;
    let target_path = save_dir.join(&file_meta.name);
    let mut file = File::create(&target_path)
        .await
        .map_err(|err| format!("创建文件失败 {}: {err}", target_path.display()))?;

    let mut received = 0_u64;
    let mut stream = body.into_data_stream();

    // axum 的 Body 是一个异步流。这里逐块写入文件，避免把大文件完整读进内存。
    // 每个 chunk 都是网络层到达的一段字节；真实大小由 hyper/axum 决定，我们只需要
    // 处理“成功拿到字节 -> 写入磁盘 -> 更新进度”这条链路。
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|err| format!("读取上传数据失败: {err}"))?;
        file.write_all(&chunk)
            .await
            .map_err(|err| format!("写入文件失败 {}: {err}", target_path.display()))?;
        received += chunk.len() as u64;

        let percent = if file_meta.size == 0 {
            100
        } else {
            ((received.saturating_mul(100) / file_meta.size).min(100)) as u8
        };
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
            },
        );
    }

    file.flush()
        .await
        .map_err(|err| format!("刷新文件失败 {}: {err}", target_path.display()))?;
    Ok(())
}

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
