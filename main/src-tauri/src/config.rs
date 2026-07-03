use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::RecentDevice;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub device_id: String,
    pub custom_device_name: Option<String>,
    pub selected_ip: Option<String>,
    pub save_dir: Option<String>,
    pub recent_devices: Vec<RecentDevice>,
}

impl AppConfig {
    pub fn load() -> Result<Self, String> {
        let path = config_file_path()?;
        if !path.exists() {
            let config = Self {
                device_id: Uuid::new_v4().to_string(),
                ..Self::default()
            };
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)
            .map_err(|err| format!("读取配置失败 {}: {err}", path.display()))?;
        let mut config: Self =
            serde_json::from_str(&content).map_err(|err| format!("解析配置失败: {err}"))?;

        if config.device_id.trim().is_empty() {
            config.device_id = Uuid::new_v4().to_string();
            config.save()?;
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_file_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("创建配置目录失败 {}: {err}", parent.display()))?;
        }

        let content =
            serde_json::to_string_pretty(self).map_err(|err| format!("序列化配置失败: {err}"))?;
        fs::write(&path, content).map_err(|err| format!("写入配置失败 {}: {err}", path.display()))
    }
}

pub fn config_dir() -> Result<PathBuf, String> {
    let mut dir = dirs::config_dir().ok_or("无法定位系统配置目录".to_string())?;
    dir.push("LanTransfer");
    Ok(dir)
}

fn config_file_path() -> Result<PathBuf, String> {
    let mut path = config_dir()?;
    path.push("config.json");
    Ok(path)
}

pub fn default_save_dir() -> Result<PathBuf, String> {
    let mut dir = dirs::download_dir().ok_or("无法定位系统下载目录".to_string())?;
    dir.push("LanTransfer");
    Ok(dir)
}

pub fn ensure_writable_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path)
        .map_err(|err| format!("创建保存目录失败 {}: {err}", path.display()))?;

    // 这个探针文件用于确认目录是否真的可写。仅检查 exists() 不够，因为目录可能存在但
    // 当前用户没有写入权限；提前发现能避免接收文件时才失败。
    let probe = path.join(".lan_transfer_write_test");
    fs::write(&probe, b"ok").map_err(|err| format!("保存目录不可写 {}: {err}", path.display()))?;
    let _ = fs::remove_file(probe);
    Ok(())
}
