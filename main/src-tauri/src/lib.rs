mod commands;
mod config;
mod device;
mod http_server;
mod models;
mod transfer;

use std::sync::Arc;

use tauri::Manager;

use crate::transfer::AppState;

// run 是 Tauri 应用的主装配函数。
// 这里负责注册插件、初始化共享状态、启动本地 HTTP 服务，并暴露所有前端可调用的 command。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // tauri_plugin_opener 用于打开外部资源， 比如在我们的程序中， 前端有一个打开保存文件夹目录的功能。
        // 有了这个plugin ，前端就直接`import { open } from "@tauri-apps/plugin-opener";` 打开
        // 但是目前的项目，仍然是使用的Command::new 打开的资源管理器
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            //app ： 当前启动的Tauri 应用的实例，但是仅在setup生命周期中可用。
            // 由于后续，我们需要在http 服务中，使用到AppHandle, 所以这里先clone一份
            // 如果是在tauri:command的handler函数中， tauri可以自动注入AppHandler, 但这里不是。
            let app_handle = app.handle().clone();
            // AppState 用于管理整个应用的context, AppState::load则是调用transfer.rs中的初始化功能
            // 由于AppState是一个context， 就意味着，在前端调用的tauri command中会被调用，以及http服务中也要被调用。
            // 但是普通变量只能有一个onwer, 因此， Arc 就像是一个智能指针，创建一份 AppState，然后用 Arc 包起来，让多个异步任务、多个 handler 都能安全地拿到它。
            let state = Arc::new(AppState::load()?);
            //Arc 解决了，多个地方访问同一个状态。
            //而Mutex 解决， 多个地方想同时修改状态，会不会乱，因此在AppState里，并不是直接存放的普通变量，而是被Mutex包装后的变量
            //Mutex 会保护，正在被修改的变量，谁要改变量，先拿锁。拿到锁的人改完再放开，其他人排队。

            // manage 把状态注册进 Tauri，之后 command 可以通过 State<Arc<AppState>> 取到它。
            // 以后前端调用 Rust command 的时候，你可以把这个 state 自动传给 command。 
            // 这里clone的不是state本身，而是Arc
            app.manage(state.clone());

            // Tauri 的 setup 在窗口创建阶段执行一次，很适合启动“跟应用生命周期一致”的后台服务。
            // 这里使用 tauri::async_runtime::spawn，而不是 tokio::spawn，是因为 Tauri 已经帮我们
            // 管理了运行时；直接复用它能避免在桌面应用里再创建一套 runtime。
            // async move 用于将外部变量的控制权，移动到这个异步任务代码块中， 否则setup一结束， state, app_handler 就没了
            tauri::async_runtime::spawn(async move {
                if let Err(err) = http_server::start_local_service(state, app_handle).await {
                    eprintln!("failed to start local HTTP service: {err}");
                }
            });

            Ok(())
        })
        // invoke_handler 注册所有 #[tauri::command] 函数。
        // 前端通过 invoke("command_name", args) 进入这些 Rust 函数。
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
