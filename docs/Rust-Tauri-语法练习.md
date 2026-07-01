# Rust + Tauri 语法练习

这些练习围绕后续局域网文件传输工具会用到的 Rust 语法设计。建议按顺序完成。

## 练习 1：变量和函数

目标：熟悉函数参数、返回值、`&str` 与 `String`。

实现函数：

```rust
fn format_device_name(name: &str) -> String {
    // 返回 "Device: xxx"
}
```

期望：

```rust
format_device_name("ThinkBook")
// "Device: ThinkBook"
```

提示：

```rust
format!("Device: {}", name)
```

## 练习 2：struct

目标：熟悉结构体定义和实例创建。

定义：

```rust
struct DeviceInfo {
    device_id: String,
    device_name: String,
    receive_enabled: bool,
}
```

实现函数：

```rust
fn create_device_info() -> DeviceInfo {
    // 返回一个默认设备
}
```

期望字段：

```text
device_id: "demo-device-id"
device_name: "ThinkBook"
receive_enabled: true
```

## 练习 3：Option

目标：理解 `Some` / `None` 和默认值逻辑。

实现函数：

```rust
fn get_display_name(custom_name: Option<String>, system_name: String) -> String {
    // 如果 custom_name 有值，返回 custom_name
    // 否则返回 system_name
}
```

期望：

```rust
get_display_name(Some("My PC".to_string()), "DESKTOP-123".to_string())
// "My PC"

get_display_name(None, "DESKTOP-123".to_string())
// "DESKTOP-123"
```

可以用 `match` 实现，也可以尝试 `unwrap_or`。

## 练习 4：Result

目标：理解成功/失败返回。

实现函数：

```rust
fn validate_port(port: u16) -> Result<u16, String> {
    // 7788-7888 返回 Ok(port)
    // 其他返回 Err("port out of range")
}
```

期望：

```rust
validate_port(7788)
// Ok(7788)

validate_port(8000)
// Err("port out of range")
```

## 练习 5：Vec

目标：熟悉列表、引用遍历、聚合计算。

定义：

```rust
struct FileInfo {
    name: String,
    size: u64,
}
```

实现函数：

```rust
fn total_size(files: &[FileInfo]) -> u64 {
    // 返回所有文件大小之和
}
```

期望：

```rust
let files = vec![
    FileInfo { name: "a.txt".to_string(), size: 100 },
    FileInfo { name: "b.zip".to_string(), size: 200 },
];

total_size(&files)
// 300
```

## 练习 6：enum + match

目标：熟悉 enum 和 match。

定义：

```rust
enum TransferStatus {
    Pending,
    Accepted,
    Rejected,
    Uploading,
    Completed,
    Failed,
}
```

实现函数：

```rust
fn status_text(status: TransferStatus) -> &'static str {
    // 返回中文状态
}
```

期望：

```text
Pending   -> "等待确认"
Accepted  -> "已接收"
Rejected  -> "已拒绝"
Uploading -> "传输中"
Completed -> "已完成"
Failed    -> "失败"
```

## 练习 7：PathBuf

目标：熟悉跨平台路径拼接。

实现函数：

```rust
use std::path::PathBuf;

fn build_save_path(download_dir: PathBuf, file_name: &str) -> PathBuf {
    // 返回 download_dir / LanTransfer / file_name
}
```

提示：

```rust
let mut path = download_dir;
path.push("LanTransfer");
path.push(file_name);
path
```

## 练习 8：Tauri command

目标：打通 React 调用 Rust 的链路。

在 `hello world/src-tauri/src/lib.rs` 中新增 command：

```rust
#[tauri::command]
fn get_device_name() -> String {
    "ThinkBook".to_string()
}
```

并注册到 `invoke_handler`：

```rust
.invoke_handler(tauri::generate_handler![greet, get_device_name])
```

在前端调用：

```ts
import { invoke } from "@tauri-apps/api/core";

const name = await invoke<string>("get_device_name");
```

目标：

```text
页面上展示 Rust 返回的设备名。
```

## 练习 9：Tauri command 返回 struct

目标：熟悉 `serde::Serialize` 和结构化返回值。

在 Rust 中定义：

```rust
use serde::Serialize;

#[derive(Serialize)]
struct DeviceInfo {
    device_id: String,
    device_name: String,
    version: String,
    receive_enabled: bool,
}
```

新增 command：

```rust
#[tauri::command]
fn get_device_info() -> DeviceInfo {
    DeviceInfo {
        device_id: "demo-device-id".to_string(),
        device_name: "ThinkBook".to_string(),
        version: "0.1.0".to_string(),
        receive_enabled: true,
    }
}
```

前端类型：

```ts
type DeviceInfo = {
  device_id: string;
  device_name: string;
  version: string;
  receive_enabled: boolean;
};
```

调用：

```ts
const info = await invoke<DeviceInfo>("get_device_info");
```

目标：

```text
页面展示 device_name、version、receive_enabled。
```

## 练习 10：Tauri command 返回 Result

目标：熟悉 Rust 错误如何传给前端。

实现：

```rust
#[tauri::command]
fn validate_device_name(name: String) -> Result<String, String> {
    // 如果 name 为空，返回 Err("device name cannot be empty")
    // 否则返回 Ok(name)
}
```

前端：

```ts
try {
  const name = await invoke<string>("validate_device_name", { name: input });
  console.log(name);
} catch (err) {
  console.error(err);
}
```

目标：

```text
输入空字符串时，前端能捕获错误。
输入非空字符串时，前端显示返回值。
```

## 练习 11：所有权观察

目标：亲自看到 move 报错。

尝试运行：

```rust
fn print_name(name: String) {
    println!("{}", name);
}

fn main() {
    let name = String::from("ThinkBook");
    print_name(name);
    println!("{}", name);
}
```

观察编译错误。

然后改成引用版本：

```rust
fn print_name(name: &str) {
    println!("{}", name);
}

fn main() {
    let name = String::from("ThinkBook");
    print_name(&name);
    println!("{}", name);
}
```

目标：

```text
理解传 String 会移动所有权，传 &str 只是借用。
```

## 练习 12：小综合练习

目标：模拟 PRD 中的设备名优先级。

实现函数：

```rust
fn resolve_device_name(custom_name: Option<String>, system_name: Option<String>) -> String {
    // 优先级：
    // 用户自定义设备名 > 系统设备名 > "My Device"
}
```

期望：

```rust
resolve_device_name(Some("ThinkBook".to_string()), Some("DESKTOP-123".to_string()))
// "ThinkBook"

resolve_device_name(None, Some("DESKTOP-123".to_string()))
// "DESKTOP-123"

resolve_device_name(None, None)
// "My Device"
```

建议分别用两种方式实现：

1. `match`
2. `or` / `unwrap_or`

