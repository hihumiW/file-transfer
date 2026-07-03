use std::{
    net::IpAddr,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use gethostname::gethostname;

use crate::{
    config::{ensure_writable_dir, AppConfig},
    models::{
        DeviceInfo, NetworkAddress, NetworkKind, RecentDevice, APP_VERSION, DEVICE_NAME_MAX_LEN,
        PROTOCOL_VERSION,
    },
};

// now_millis 统一生成前端可直接使用的毫秒时间戳。
// 用 UNIX_EPOCH 是为了让 Rust、TypeScript 和 JSON 之间的时间表达保持简单。
pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

// validate_device_name 集中处理设备名规则：去掉首尾空白、禁止空名、限制最大长度。
// 这样保存自定义名称和解析已有配置时可以复用同一套校验逻辑。
pub fn validate_device_name(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("设备名称不能为空".to_string());
    }
    if trimmed.chars().count() > DEVICE_NAME_MAX_LEN {
        return Err(format!("设备名称不能超过 {DEVICE_NAME_MAX_LEN} 个字符"));
    }
    Ok(trimmed.to_string())
}

// resolved_device_name 实现 PRD 中的优先级：
// 用户自定义设备名 > 系统设备名 > "My Device" 兜底名称。
pub fn resolved_device_name(config: &AppConfig) -> String {
    if let Some(name) = &config.custom_device_name {
        if let Ok(valid_name) = validate_device_name(name) {
            return valid_name;
        }
    }

    let hostname = gethostname();
    if !hostname.is_empty() {
        return hostname.to_string_lossy().trim().to_string();
    }

    "My Device".to_string()
}

// build_device_info 生成暴露给本机 UI 和远端 GET /api/device 的设备信息。
// receive_enabled 在这里通过保存目录可写性计算，保证“能否接收”反映真实文件系统状态。
pub fn build_device_info(config: &AppConfig, save_dir: &PathBuf) -> DeviceInfo {
    DeviceInfo {
        ok: true,
        device_id: config.device_id.clone(),
        device_name: resolved_device_name(config),
        version: APP_VERSION.to_string(),
        protocol_version: PROTOCOL_VERSION.to_string(),
        receive_enabled: ensure_writable_dir(save_dir).is_ok(),
    }
}

// list_network_addresses 枚举本机 IPv4 候选地址，并按“更可能是局域网可达地址”的分数排序。
// 它只决定 UI 展示和复制地址；HTTP 服务仍监听 0.0.0.0，接收所有网卡连接。
pub fn list_network_addresses() -> Vec<NetworkAddress> {
    let mut addresses = local_ip_address::list_afinet_netifas()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(name, ip)| match ip {
            IpAddr::V4(v4) if is_candidate_ipv4(&v4.octets()) => {
                let kind = classify_interface(&name);
                let score = score_interface(&name, &v4.octets(), &kind);
                Some((score, name, v4.to_string(), kind))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    addresses.sort_by(|left, right| right.0.cmp(&left.0).then(left.1.cmp(&right.1)));

    let recommended_ip = addresses.first().map(|(_, _, ip, _)| ip.clone());

    addresses
        .into_iter()
        .map(|(_, name, ip, kind)| NetworkAddress {
            recommended: recommended_ip.as_deref() == Some(ip.as_str()),
            label: name,
            ip,
            kind,
        })
        .collect()
}

// is_candidate_ipv4 过滤明显不适合给其他设备连接的地址。
// loopback 只能本机访问，169.254.* 是链路本地地址，通常代表没有正常拿到局域网地址。
fn is_candidate_ipv4(octets: &[u8; 4]) -> bool {
    if octets[0] == 127 {
        return false;
    }
    if octets[0] == 169 && octets[1] == 254 {
        return false;
    }
    true
}

// is_private_ipv4 识别 RFC1918 私有地址段。
// 局域网传输最常见的地址就在这些网段内，所以评分时会优先它们。
fn is_private_ipv4(octets: &[u8; 4]) -> bool {
    octets[0] == 10
        || (octets[0] == 192 && octets[1] == 168)
        || (octets[0] == 172 && (16..=31).contains(&octets[1]))
}

// classify_interface 根据网卡名称做启发式分类。
// 不同操作系统的网卡命名不完全一致，所以这里追求“够用可解释”，不追求绝对准确。
fn classify_interface(name: &str) -> NetworkKind {
    let lower = name.to_ascii_lowercase();
    if lower.contains("wi-fi") || lower.contains("wifi") || lower.contains("wlan") {
        NetworkKind::Wifi
    } else if lower.contains("ethernet") || lower.contains("eth") || lower.contains("en") {
        NetworkKind::Ethernet
    } else if lower.contains("vpn") {
        NetworkKind::Vpn
    } else if lower.contains("docker")
        || lower.contains("wsl")
        || lower.contains("virtual")
        || lower.contains("vmware")
        || lower.contains("virtualbox")
        || lower.contains("hyper-v")
        || lower.contains("vethernet")
    {
        NetworkKind::Virtual
    } else {
        NetworkKind::Other
    }
}

// score_interface 把候选 IP 排成推荐顺序。
// 私有地址、Wi-Fi 和有线网卡加分；VPN、虚拟网卡和 loopback 名称降权。
fn score_interface(name: &str, octets: &[u8; 4], kind: &NetworkKind) -> i32 {
    let mut score = 0;
    if is_private_ipv4(octets) {
        score += 100;
    }
    score += match kind {
        NetworkKind::Wifi => 40,
        NetworkKind::Ethernet => 35,
        NetworkKind::Other => 10,
        NetworkKind::Vpn => -20,
        NetworkKind::Virtual => -40,
    };

    let lower = name.to_ascii_lowercase();
    if lower.contains("loopback") {
        score -= 100;
    }
    score
}

// upsert_recent_device 保存连接成功的目标设备。
// 优先按 device_id 更新同一设备；如果没有 device_id，则按规范化后的 address 更新。
pub fn upsert_recent_device(recent_devices: &mut Vec<RecentDevice>, mut next: RecentDevice) {
    next.last_success_at = Some(next.last_connected_at);

    if let Some(index) = recent_devices.iter().position(|item| {
        next.device_id
            .as_ref()
            .zip(item.device_id.as_ref())
            .map(|(left, right)| left == right)
            .unwrap_or(false)
            || item.address == next.address
    }) {
        recent_devices[index] = next;
    } else {
        recent_devices.push(next);
    }

    recent_devices.sort_by(|left, right| right.last_connected_at.cmp(&left.last_connected_at));
    recent_devices.truncate(10);
}
