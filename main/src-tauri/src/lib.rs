mod commands;
mod config;
mod device;
mod http_server;
mod models;
mod transfer;

use std::sync::Arc;

use tauri::Manager;

use crate::transfer::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = Arc::new(AppState::load()?);

            app.manage(state.clone());

            // Tauri 的 setup 在窗口创建阶段执行一次，很适合启动“跟应用生命周期一致”的后台服务。
            // 这里使用 tauri::async_runtime::spawn，而不是 tokio::spawn，是因为 Tauri 已经帮我们
            // 管理了运行时；直接复用它能避免在桌面应用里再创建一套 runtime。
            tauri::async_runtime::spawn(async move {
                if let Err(err) = http_server::start_local_service(state, app_handle).await {
                    eprintln!("failed to start local HTTP service: {err}");
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_snapshot,
            commands::save_device_name,
            commands::select_display_ip,
            commands::choose_save_dir,
            commands::open_save_dir,
            commands::choose_files,
            commands::describe_paths,
            commands::normalize_target_address,
            commands::test_target_connection,
            commands::delete_recent_device,
            commands::send_files,
            commands::respond_transfer,
            commands::clear_completed_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
