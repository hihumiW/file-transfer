use std::{path::PathBuf, sync::Mutex};

use crate::{
    config::{default_save_dir, ensure_writable_dir, AppConfig},
    models::{PendingTransfer, ServiceStatus, TransferTask, DEFAULT_PORT},
};

// AppState 是 Rust 端的共享运行时状态。
// Tauri command 和 axum HTTP handler 会从不同异步任务访问它，所以每个可变字段都放在 Mutex 中。
pub struct AppState {
    // config 是持久化配置的内存副本；修改后需要显式调用 config.save() 写回磁盘。
    pub config: Mutex<AppConfig>,
    // service 记录本地 HTTP 接收服务的状态和实际端口，供前端快照展示。
    pub service: Mutex<ServiceStatus>,
    // tasks 是当前应用运行期间的任务列表；v0.1 不把它写入配置或数据库。
    pub tasks: Mutex<Vec<TransferTask>>,
    // pending_transfer 表示接收端当前待确认或传输中的单个任务，用来实现“同一时间只接收一个任务”。
    pub pending_transfer: Mutex<Option<PendingTransfer>>,
}

impl AppState {
    // load 在 Tauri setup 阶段调用，完成配置读取和运行期状态初始化。
    // 这里不启动 HTTP 服务，只准备状态；服务启动在 lib.rs 中单独 spawn。
    pub fn load() -> Result<Self, String> {
        Ok(Self {
            config: Mutex::new(AppConfig::load()?),
            service: Mutex::new(ServiceStatus {
                running: false,
                port: DEFAULT_PORT,
                message: "本地接收服务启动中".to_string(),
            }),
            tasks: Mutex::new(Vec::new()),
            pending_transfer: Mutex::new(None),
        })
    }

    // save_dir 返回当前应该使用的保存目录。
    // 用户设置过自定义目录时优先使用，否则回退到系统下载目录/LanTransfer。
    pub fn save_dir(&self) -> Result<PathBuf, String> {
        let config = self.config.lock().map_err(|_| "配置锁已损坏".to_string())?;
        if let Some(save_dir) = &config.save_dir {
            return Ok(PathBuf::from(save_dir));
        }
        default_save_dir()
    }

    // ensure_save_dir 在真正接收文件前调用，保证目录存在且可写。
    // 返回 PathBuf 是为了调用方可以立即把文件写到这个目录下。
    pub fn ensure_save_dir(&self) -> Result<PathBuf, String> {
        let save_dir = self.save_dir()?;
        ensure_writable_dir(&save_dir)?;
        Ok(save_dir)
    }

    // set_service_running 由 HTTP 服务成功绑定端口后调用。
    // 端口可能不是默认 7788，因为启动逻辑会自动尝试 7788-7888。
    pub fn set_service_running(&self, port: u16) -> Result<(), String> {
        let mut service = self
            .service
            .lock()
            .map_err(|_| "服务状态锁已损坏".to_string())?;
        service.running = true;
        service.port = port;
        service.message = "正在运行".to_string();
        Ok(())
    }

    // set_service_error 记录服务启动失败原因，让前端能展示明确错误。
    pub fn set_service_error(&self, message: String) -> Result<(), String> {
        let mut service = self
            .service
            .lock()
            .map_err(|_| "服务状态锁已损坏".to_string())?;
        service.running = false;
        service.message = message;
        Ok(())
    }
}
