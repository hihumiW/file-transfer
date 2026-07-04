use serde::{Deserialize, Serialize};

// 这些常量是 Rust 端和前端、设备间 HTTP 协议共同依赖的“固定约定”。
// 集中放在 models.rs 中，可以避免不同模块各自硬编码版本号、端口和校验规则。
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_VERSION: &str = "0.1";
pub const DEFAULT_PORT: u16 = 7788;
pub const MAX_PORT: u16 = 7888;
pub const DEVICE_NAME_MAX_LEN: usize = 256;

// DeviceInfo 对应 GET /api/device 的响应，也会出现在前端快照中。
// serde(rename_all = "camelCase") 让 Rust 的 snake_case 字段自动转成前端习惯的 camelCase。
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

// NetworkAddress 是 UI 展示的本机候选地址。
// recommended 只影响界面默认展示，不改变 HTTP 服务实际监听的 0.0.0.0。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkAddress {
    pub ip: String,
    pub label: String,
    pub kind: NetworkKind,
    pub recommended: bool,
}

// NetworkKind 用来表达网卡类型。
// 它不是系统级精确分类，而是根据网卡名称做的 MVP 级启发式判断。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NetworkKind {
    Wifi,
    Ethernet,
    Vpn,
    Virtual,
    Other,
}

// ServiceStatus 记录本地 axum 接收服务的运行状态。
// 前端顶部状态区通过它展示“正在运行 / 启动失败 / 实际端口”。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    pub running: bool,
    pub port: u16,
    pub message: String,
}

// RecentDevice 是持久化到本地配置中的最近连接记录。
// device_id 优先用于识别同一台设备；没有 device_id 时退回用 address 去重。
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

// LocalFile 描述本机待发送路径。
// 这里保留 is_dir，是为了在发送前明确拒绝文件夹，符合 v0.1 不支持目录传输的范围。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

// TransferFile 是设备间传输协议中的文件元数据，只包含接收方确认所需的信息。
// 真正的文件内容会在 /api/transfer/upload 中以 HTTP body 流式传输。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFile {
    pub name: String,
    pub size: u64,
}

// TransferDirection 表示任务方向，前端用它生成“发送给 / 从...接收”的文案。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferDirection {
    Send,
    Receive,
}

// TransferStatus 是传输任务的核心状态机。
// 发送端轮询接收端状态时，也会复用同一组状态，保证两端语义一致。
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

// TransferTask 是运行期任务列表中的一条记录。
// v0.1 不做历史持久化，因此它只存放在 AppState.tasks 的内存 Vec 中。
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

// PendingTransfer 表示接收端“正在等待用户处理”的请求。
// 它比 TransferTask 多了 overwrite_confirmed 和 duplicate_files，用于接收确认弹窗。
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

// TransferRequest 是发送端提交给接收端的元数据请求。
// 注意它不包含文件内容；这样接收方可以先确认，再决定是否允许上传。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub sender_device_id: Option<String>,
    pub sender_device_name: String,
    pub files: Vec<TransferFile>,
    pub total_bytes: u64,
}

// TransferRequestResponse 是 POST /api/transfer/request 的响应。
// 成功时返回接收端生成的 transfer_id；失败时返回 error_code 和 message 给发送端展示。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequestResponse {
    pub ok: bool,
    pub transfer_id: Option<String>,
    pub status: Option<TransferStatus>,
    pub error_code: Option<String>,
    pub message: Option<String>,
}

// TransferStatusResponse 是发送端轮询 GET /api/transfer/status/:transferId 的响应。
// 轮询可以让 v0.1 避免引入 WebSocket/SSE，同时仍能等待接收方确认。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferStatusResponse {
    pub ok: bool,
    pub transfer_id: String,
    pub status: TransferStatus,
    pub message: Option<String>,
}

// AppSnapshot 是前端一次性刷新 UI 所需的完整快照。
// 多数 Tauri command 执行后返回它，让 React/Zustand 不必分别请求多个 Rust 状态。
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

// TargetConnection 是连接测试成功后的结果。
// 它会成为前端“当前发送目标”，并写入最近连接设备列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetConnection {
    pub device: DeviceInfo,
    pub address: String,
    pub ip: String,
    pub port: u16,
}

// ProgressEvent 是 Rust 通过 Tauri event 推送给前端的进度事件。
// 发送和接收都会使用同名事件，前端根据 transfer_id 更新对应任务。
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransferStatus>,
}

// IncomingTransferEvent 是接收端收到传输请求后通知前端弹出确认 UI 的事件。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingTransferEvent {
    pub transfer: PendingTransfer,
}
