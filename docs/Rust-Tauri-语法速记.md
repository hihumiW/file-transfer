# Rust + Tauri 语法速记

本文面向有 TypeScript/前端基础的开发者，目标是快速理解后续 Tauri 应用开发中高频使用的 Rust 语法。

## 1. Rust 的基本心智模型

可以先这样理解：

```text
TypeScript:
写代码 -> 运行时可能报错

Rust:
写代码 -> 编译器先严格检查 -> 通过后运行时更稳
```

Rust 的特点：

- 静态类型。
- 默认不可变。
- 没有 `null`。
- 错误显式处理。
- 有所有权系统。
- 编译成原生二进制。

在 Tauri 应用中，Rust 常用于：

- Tauri command。
- 文件读写。
- 路径处理。
- 本地配置。
- HTTP 服务。
- 进度事件。
- 系统能力调用。

## 2. 常见数据类型

### 2.1 整数类型

`i32` 表示有符号 32 位整数，可以为负数：

```rust
let a: i32 = -10;
```

`u64` 表示无符号 64 位整数，不能为负数：

```rust
let size: u64 = 1024;
```

命名规则：

```text
i = signed integer，有正负
u = unsigned integer，无负数
数字 = 位数
```

常见整数类型：

```text
i8   i16   i32   i64   i128   isize
u8   u16   u32   u64   u128   usize
```

常用选择：

```text
普通数字：i32
文件大小、字节数：u64
数组下标、长度：usize
端口号：u16
```

### 2.2 其他常见类型

```rust
bool        // true / false
char        // 单个 Unicode 字符
f32 / f64   // 小数
String      // 拥有所有权、可变长字符串
&str        // 字符串切片/引用
Vec<T>      // 列表
Option<T>   // 有值/无值
Result<T,E> // 成功/失败
struct      // 结构体
enum        // 枚举
```

## 3. 变量默认不可变

TypeScript：

```ts
const name = "ThinkBook";
let count = 1;
count += 1;
```

Rust：

```rust
let name = "ThinkBook";
let mut count = 1;
count += 1;
```

Rust 的 `let` 默认不可变：

```rust
let count = 1;
count += 1; // 编译错误
```

如果要修改，需要 `mut`：

```rust
let mut count = 1;
count += 1;
```

## 4. `&str` 与 `String`

直接写字符串字面量：

```rust
let name = "ThinkBook";
```

它的类型通常是：

```rust
&str
```

可以先这样理解：

```text
&str   = 指向一段字符串内容的只读引用
String = 真正拥有一段字符串内容的对象
```

示例：

```rust
let a: &str = "hello";
let b: String = String::from("hello");
let c: String = "hello".to_string();
```

如果要修改字符串，需要 `String`，并且变量要声明为 `mut`：

```rust
let mut name = String::from("Think");
name.push_str("Book");
name.push('!');
```

常用字符串方法：

```rust
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');
s.clear();
```

也可以使用 `format!` 创建新字符串：

```rust
let name = "ThinkBook";
let text = format!("Device: {}", name);
```

## 5. 函数与返回值

TypeScript：

```ts
function add(a: number, b: number): number {
  return a + b;
}
```

Rust：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Rust 最后一行没有分号时，作为返回值：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

如果加分号，就变成语句，不再作为返回值：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b; // 编译错误
}
```

显式返回也可以：

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

没有返回值的函数返回 `()`，类似 TypeScript 的 `void`：

```rust
fn say_hello() {
    println!("hello");
}
```

等价于：

```rust
fn say_hello() -> () {
    println!("hello");
}
```

## 6. struct

TypeScript：

```ts
type DeviceInfo = {
  deviceId: string;
  deviceName: string;
  receiveEnabled: boolean;
};
```

Rust：

```rust
struct DeviceInfo {
    device_id: String,
    device_name: String,
    receive_enabled: bool,
}
```

创建实例：

```rust
let device = DeviceInfo {
    device_id: "abc".to_string(),
    device_name: "ThinkBook".to_string(),
    receive_enabled: true,
};
```

访问字段：

```rust
println!("{}", device.device_name);
```

Rust 命名习惯：

```text
变量、字段、函数：snake_case
类型、struct、enum：PascalCase
```

如果要返回 JSON，常配合 `serde`：

```rust
use serde::Serialize;

#[derive(Serialize)]
struct DeviceInfo {
    device_id: String,
    device_name: String,
    receive_enabled: bool,
}
```

## 7. enum 与 match

Rust 的 `enum` 比 TypeScript 的普通 enum 更强。

类似 TypeScript：

```ts
type TransferStatus = "pending" | "accepted" | "failed";
```

Rust：

```rust
enum TransferStatus {
    Pending,
    Accepted,
    Failed,
}
```

也可以带数据：

```rust
enum TransferResult {
    Success,
    Failed(String),
}
```

`match` 可以理解为更强的 `switch`：

```rust
let number = 2;

let text = match number {
    1 => "one",
    2 => "two",
    _ => "other",
};
```

`_` 表示其他所有情况，类似 `default`。

处理 enum：

```rust
enum Status {
    Pending,
    Done,
}

fn status_text(status: Status) -> &'static str {
    match status {
        Status::Pending => "等待",
        Status::Done => "完成",
    }
}
```

处理带数据的 enum：

```rust
let result = TransferResult::Failed("network error".to_string());

match result {
    TransferResult::Success => println!("ok"),
    TransferResult::Failed(reason) => println!("failed: {}", reason),
}
```

## 8. Option

Rust 没有 `null`，用 `Option<T>` 表示“可能有值，也可能没有值”。

可以理解为：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

含义：

```text
Some(value) = 有值
None        = 没有值
```

TypeScript 类比：

```ts
string | undefined
```

Rust：

```rust
let name1: Option<String> = Some("ThinkBook".to_string());
let name2: Option<String> = None;
```

使用 `match` 处理：

```rust
match name1 {
    Some(value) => println!("有名字：{}", value),
    None => println!("没有名字"),
}
```

这里的 `Some(value)` 不是函数调用，而是模式匹配：

```text
如果这个 Option 是 Some(...)，就把里面的值绑定到 value。
```

简写：

```rust
if let Some(value) = name2 {
    println!("{}", value);
}
```

含义：

```text
如果 name2 是 Some，就取出里面的值叫 value，然后执行代码块。
如果是 None，就什么都不做。
```

带 else：

```rust
if let Some(value) = name2 {
    println!("有值：{}", value);
} else {
    println!("没有值");
}
```

常用方法：

```rust
let display_name = custom_name.unwrap_or(system_name);
```

含义：

```text
custom_name 有值就用它，否则用 system_name。
```

## 9. Result

Rust 用 `Result<T, E>` 表示成功或失败。

```rust
Result<T, E>
```

含义：

```text
Ok(value) = 成功，里面是结果
Err(error) = 失败，里面是错误
```

示例：

```rust
fn read_config() -> Result<String, String> {
    Ok("config".to_string())
}
```

使用 `match` 处理：

```rust
match read_config() {
    Ok(value) => println!("{}", value),
    Err(err) => println!("error: {}", err),
}
```

Tauri command 中常见：

```rust
#[tauri::command]
fn get_save_dir() -> Result<String, String> {
    Ok("C:/Users/xxx/Downloads/LanTransfer".to_string())
}
```

前端调用时：

```ts
try {
  const dir = await invoke<string>("get_save_dir");
} catch (err) {
  console.error(err);
}
```

`?` 操作符：

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
```

含义：

```text
如果成功，取出 Ok 里的值。
如果失败，直接 return Err。
```

## 10. 所有权、引用与借用

Rust 中，每个值在同一时刻只能有一个 owner。owner 离开作用域时，这个值会被释放。

```rust
{
    let name = String::from("ThinkBook");
} // name 离开作用域，字符串内存被释放
```

TypeScript/JavaScript 靠 GC 管理内存。Rust 没有 GC，靠所有权在编译期决定什么时候释放。

### 10.1 移动所有权

```rust
let a = String::from("hello");
let b = a;

println!("{}", b); // OK
println!("{}", a); // 编译错误
```

`let b = a;` 不是复制字符串内容，而是把所有权从 `a` 移动给 `b`。

这叫 move。

### 10.2 消耗所有权

```rust
fn print_name(name: String) {
    println!("{}", name);
}

let a = String::from("ThinkBook");
print_name(a);
println!("{}", a); // 编译错误
```

调用 `print_name(a)` 时，`a` 的所有权被传进函数。函数执行完后，里面的 `name` 被释放，外面的 `a` 不能再使用。

这就是“消耗所有权”。

### 10.3 引用与借用

如果不想把所有权交出去，就传引用：

```rust
fn print_name(name: &String) {
    println!("{}", name);
}

let a = String::from("ThinkBook");
print_name(&a);
println!("{}", a); // OK
```

`&a` 表示：

```text
借用 a，不拿走 a。
```

### 10.4 `&str` 与 `&变量名`

`&变量名` 是取引用的语法：

```rust
let name = String::from("hello");
let r = &name;
```

`&str` 是一种引用类型：

```rust
let s: &str = "hello";
```

可以理解为：

```text
&name = 创建一个引用
&str  = 字符串引用的类型
```

字符串字面量：

```rust
let s = "hello";
```

本质上是一个 `&'static str`，指向程序二进制里的静态字符串区域。

从 `String` 也能借出 `&str`：

```rust
let name = String::from("ThinkBook");
let s: &str = &name;
```

### 10.5 为什么只读参数优先用 `&str`

如果函数只是读取字符串，不应该拿走所有权：

```rust
fn print_name(name: String) {
    println!("{}", name);
}

let n = String::from("ThinkBook");
print_name(n);
println!("{}", n); // 编译错误
```

更推荐：

```rust
fn print_name(name: &str) {
    println!("{}", name);
}

let n = String::from("ThinkBook");
print_name(&n);
println!("{}", n); // OK

print_name("MacBook"); // 也 OK
```

规则：

```text
只读参数：&str
需要修改：&mut String
需要拥有/保存：String
```

示例：

```rust
fn read_only(name: &str) {}

fn modify(name: &mut String) {
    name.push_str("!");
}

fn take_ownership(name: String) {
    // 保存到结构体、放到 Vec、跨线程等
}
```

## 11. Vec

TypeScript：

```ts
const files: string[] = [];
files.push("a.txt");
```

Rust：

```rust
let mut files: Vec<String> = Vec::new();
files.push("a.txt".to_string());
```

遍历并消耗所有权：

```rust
for file in files {
    println!("{}", file);
}
```

只读遍历：

```rust
for file in &files {
    println!("{}", file);
}
```

文件列表示例：

```rust
struct TransferRequest {
    files: Vec<FileInfo>,
}
```

## 12. HashMap

TypeScript：

```ts
const map = new Map<string, string>();
map.set("id", "abc");
```

Rust：

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("id".to_string(), "abc".to_string());
```

传输任务状态中可能会用：

```rust
HashMap<String, TransferTask>
```

含义：

```text
transferId -> transferTask
```

## 13. async / await

TypeScript：

```ts
async function getDevice() {
  const res = await fetch("/api/device");
}
```

Rust：

```rust
async fn get_device() -> Result<String, String> {
    Ok("device".to_string())
}
```

Tauri command 可以是 async：

```rust
#[tauri::command]
async fn get_device_info() -> Result<String, String> {
    Ok("ThinkBook".to_string())
}
```

Axum handler 也是 async：

```rust
async fn device_info() -> Json<DeviceInfo> {
    Json(DeviceInfo {
        device_name: "ThinkBook".to_string(),
        receive_enabled: true,
    })
}
```

注意：

```text
Rust 的 async 需要运行时，比如 tokio。
```

## 14. PathBuf

路径处理不要自己拼字符串。

不推荐：

```rust
let path = format!("{}/{}", dir, file_name);
```

推荐：

```rust
use std::path::PathBuf;

let mut dir = PathBuf::from("C:/Users/xxx/Downloads");
dir.push("LanTransfer");
dir.push("demo.zip");
```

跨平台路径尤其应该用 `PathBuf`。

## 15. derive

你会经常看到：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceInfo {
    device_name: String,
}
```

含义：

```text
Debug       可以 println!("{:?}", value)
Clone       可以 value.clone()
Serialize   可以转 JSON
Deserialize 可以从 JSON 解析
```

对 Tauri/HTTP JSON 很常用。

## 16. `::` 语法

`::` 是 Rust 的路径访问符。

有点像 TypeScript 中的：

```ts
Math.random()
User.create()
Namespace.Type
```

Rust 示例：

```rust
String::from("hello")
Vec::new()
HashMap::new()
PathBuf::from("xxx")
TransferStatus::Pending
std::fs::read_to_string("a.txt")
```

区别：

```text
:: 用于模块、类型、枚举变体、关联函数。
.  用于实例方法和字段访问。
```

示例：

```rust
let mut s = String::from("hello");
//        ^^^^^^ 类型上的函数，用 ::

s.push_str(" world");
// 实例方法，用 .
```

## 17. Tauri 开发高频写法

### 17.1 command

```rust
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
```

### 17.2 返回结构体

```rust
use serde::Serialize;

#[derive(Serialize)]
struct DeviceInfo {
    device_name: String,
    version: String,
    receive_enabled: bool,
}

#[tauri::command]
fn get_device_info() -> DeviceInfo {
    DeviceInfo {
        device_name: "ThinkBook".to_string(),
        version: "0.1.0".to_string(),
        receive_enabled: true,
    }
}
```

### 17.3 返回错误

```rust
#[tauri::command]
fn read_config() -> Result<String, String> {
    std::fs::read_to_string("config.json")
        .map_err(|err| err.to_string())
}
```

## 18. 常见坑点

1. `let` 默认不可变，忘记 `mut`。
2. `String` 和 `&str` 混用报错。
3. 值被 move 后继续使用。
4. `Result` / `Option` 没处理。
5. 最后一行误加分号导致返回值不对。
6. 路径用字符串拼接，跨平台出问题。
7. 一次性读大文件进内存。
8. async 函数里用了阻塞 IO。

对文件传输项目尤其注意：

```text
不要 std::fs::read 整个大文件再上传。
不要把上传文件 Vec<u8> 全塞内存。
保存路径用 PathBuf。
错误通过 Result 返回给前端。
```

## 19. 快速对照表

| 概念 | Rust | TypeScript 类比 |
| --- | --- | --- |
| 不可变变量 | `let name` | `const name` |
| 可变变量 | `let mut name` | `let name` |
| 无返回值 | `()` | `void` |
| 无值 | `None` | `undefined/null` |
| 有值 | `Some(value)` | 真实值 |
| 成功 | `Ok(value)` | return value |
| 失败 | `Err(error)` | throw error / error result |
| 模式匹配 | `match` | 更强的 switch |
| 字符串引用 | `&str` | 只读 string view |
| 可拥有字符串 | `String` | string 对象，粗略类比 |
| 引用 | `&value` | 借用，不拿走 |
| 可变引用 | `&mut value` | 借来修改 |
| 路径访问 | `::` | namespace/type static access |
| 实例访问 | `.` | object.method / object.prop |

