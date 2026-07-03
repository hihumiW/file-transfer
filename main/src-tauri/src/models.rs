use serde::{Deserialize, Serialize};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_VERSION: &str = "0.1";
pub const DEFAULT_PORT: u16 = 7788;
pub const MAX_PORT: u16 = 7888;
pub const DEVICE_NAME_MAX_LEN: usize = 256;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub ok: bool,
    pub device_id: String,
    pub device_name: String,
    pub version: String,
    pub protocol_version: String,
    pub receive_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAddress {
    pub ip: String,
    pub label: String,
    pub kind: NetworkKind,
    pub recommended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NetworkKind {
    Wifi,
    Ethernet,
    Vpn,
    Virtual,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    pub running: bool,
    pub port: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentDevice {
    pub device_id: Option<String>,
    pub device_name: String,
    pub address: String,
    pub ip: String,
    pub port: u16,
    pub last_connected_at: i64,
    pub last_success_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFile {
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferDirection {
    Send,
    Receive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransferStatus {
    Pending,
    Accepted,
    Rejected,
    Uploading,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTask {
    pub id: String,
    pub direction: TransferDirection,
    pub peer_device_id: Option<String>,
    pub peer_device_name: String,
    pub peer_address: Option<String>,
    pub files: Vec<TransferFile>,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub status: TransferStatus,
    pub message: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingTransfer {
    pub transfer_id: String,
    pub sender_device_id: Option<String>,
    pub sender_device_name: String,
    pub sender_address: String,
    pub files: Vec<TransferFile>,
    pub total_bytes: u64,
    pub status: TransferStatus,
    pub overwrite_confirmed: bool,
    pub duplicate_files: Vec<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub sender_device_id: Option<String>,
    pub sender_device_name: String,
    pub files: Vec<TransferFile>,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequestResponse {
    pub ok: bool,
    pub transfer_id: Option<String>,
    pub status: Option<TransferStatus>,
    pub error_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferStatusResponse {
    pub ok: bool,
    pub transfer_id: String,
    pub status: TransferStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub device: DeviceInfo,
    pub display_address: String,
    pub selected_ip: Option<String>,
    pub network_addresses: Vec<NetworkAddress>,
    pub service: ServiceStatus,
    pub save_dir: String,
    pub save_dir_available: bool,
    pub recent_devices: Vec<RecentDevice>,
    pub tasks: Vec<TransferTask>,
    pub pending_transfer: Option<PendingTransfer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetConnection {
    pub device: DeviceInfo,
    pub address: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEvent {
    pub transfer_id: String,
    pub file_name: String,
    pub file_index: usize,
    pub file_total: usize,
    pub transferred_bytes: u64,
    pub total_bytes: u64,
    pub percent: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingTransferEvent {
    pub transfer: PendingTransfer,
}
