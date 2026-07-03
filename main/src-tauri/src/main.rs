// Windows 发布版默认会额外弹出控制台窗口；这个属性用于隐藏它，请不要删除。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// main.rs 是二进制入口。
// 真正的 Tauri 应用搭建放在 lib.rs，便于测试、复用和符合 Tauri 默认项目结构。
fn main() {
    lan_transfer_lib::run()
}
