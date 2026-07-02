use serde::Serialize;
use tauri::{AppHandle, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            validate_device_name,
            resolve_device_name,
            start_fake_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn validate_device_name(device_name : &str) -> Result<String, String> {
    if device_name.is_empty(){
        return Err("device name 不能为空".to_string());
    } 
    if device_name.len() > 10{
        return Err("device name 不能超过10个字符".to_string());
    }
    Ok("device name 有效".to_string())
}

#[tauri::command]
fn resolve_device_name(custom_name : Option<String>, default_name : String) -> String {
    // let result : String = match custom_name{
    //     //如果用户提供了自定义名称，则使用该名称
    //     Some(name) => name,
    //     //否则使用默认名称
    //     None => default_name,
    // };
    // // 如果 default_name 为空，则返回 "My Device"
    // Some(result).unwrap_or("My Device".to_string())

    match custom_name{
        Some(name) if !name.is_empty() => name,
        _ if !default_name.is_empty() => default_name,
        _ => "My Device".to_string()
    }
}


#[derive(Serialize, Clone)]
struct ProgressPayload {
    progress : i32
}

#[tauri::command]
async fn start_fake_task(app : AppHandle, task_name : Option<String>) -> Result<(), String>{
    println!("receive fake task name : {}", task_name.unwrap_or("no task name".to_string()));
    let precents = [0, 20, 25, 50 ,75, 100];
    for precent in precents{
        // 向前端发送事件
        app.emit("task-progress", ProgressPayload{
            progress : precent
        }).map_err(|err| err.to_string())?; 
        // 由于app.emit 返回的是Result， 因此可能返回Err(tauri::Error)， 所以这里需要使用 .map_err ，在遇到错误时将Err(tauri:Err) 转换为 Err(String)
        // ? 代表快速错误返回， 当emit执行失败了，就将错误作为返回值
        // 在官方文档中，经常会出现 app.emit().unwrap()。 .unwrap() 用于取出Result 中的值， 如果报错了，则直接panic
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if precent == 100{
            print!("done");
            app.emit("task-done", ()).map_err(|err| err.to_string())?;
        }

    }
    Ok(())
}