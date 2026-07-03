use std::{path::PathBuf, sync::Mutex};

use crate::{
    config::{default_save_dir, ensure_writable_dir, AppConfig},
    models::{PendingTransfer, ServiceStatus, TransferTask, DEFAULT_PORT},
};

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub service: Mutex<ServiceStatus>,
    pub tasks: Mutex<Vec<TransferTask>>,
    pub pending_transfer: Mutex<Option<PendingTransfer>>,
}

impl AppState {
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

    pub fn save_dir(&self) -> Result<PathBuf, String> {
        let config = self.config.lock().map_err(|_| "配置锁已损坏".to_string())?;
        if let Some(save_dir) = &config.save_dir {
            return Ok(PathBuf::from(save_dir));
        }
        default_save_dir()
    }

    pub fn ensure_save_dir(&self) -> Result<PathBuf, String> {
        let save_dir = self.save_dir()?;
        ensure_writable_dir(&save_dir)?;
        Ok(save_dir)
    }

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
