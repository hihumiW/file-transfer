use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::RecentDevice;

// AppConfig 是真正写入磁盘的本地配置。
// 这里只保存跨启动需要记住的内容；运行期任务、pending 请求等临时状态不放进配置文件。
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
    // load 负责读取配置文件；如果是首次启动，就创建一份带 device_id 的默认配置。
    // device_id 使用 UUID v4，是因为它只需要本机稳定且随机，不依赖设备名、IP 或网卡信息。
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

        // 兼容异常配置：如果用户手动删掉 device_id，启动时重新补一个，避免协议字段为空。
        if config.device_id.trim().is_empty() {
            config.device_id = Uuid::new_v4().to_string();
            config.save()?;
        }

        Ok(config)
    }

    // save 每次写完整 JSON，而不是做局部 patch。
    // 对这个小配置文件来说，完整写入更简单，也更容易人工查看和调试。
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

// config_dir 使用系统标准配置目录，避免把用户配置写进项目目录或安装目录。
pub fn config_dir() -> Result<PathBuf, String> {
    let mut dir = dirs::config_dir().ok_or("无法定位系统配置目录".to_string())?;
    dir.push("LanTransfer");
    Ok(dir)
}

// config_file_path 只在本模块内部使用，统一决定配置文件名。
fn config_file_path() -> Result<PathBuf, String> {
    let mut path = config_dir()?;
    path.push("config.json");
    Ok(path)
}

// default_save_dir 对应 PRD 中的默认保存目录：系统下载目录/LanTransfer。
// 用户没有自定义保存目录时，AppState 会回退到这个路径。
pub fn default_save_dir() -> Result<PathBuf, String> {
    let mut dir = dirs::download_dir().ok_or("无法定位系统下载目录".to_string())?;
    dir.push("LanTransfer");
    Ok(dir)
}

// ensure_writable_dir 是接收能力 receiveEnabled 的底层判断依据。
// 它既会尝试创建目录，也会写入一个临时探针文件来验证当前用户真的有写权限。
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

// is_save_dir_available 是 UI 快照使用的轻量检查。
// 高频刷新时不能写探针文件，否则资源管理器会反复看到临时文件出现和消失。
pub fn is_save_dir_available(path: &Path) -> bool {
    if fs::create_dir_all(path).is_err() {
        return false;
    }
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    metadata.is_dir() && !metadata.permissions().readonly()
}
