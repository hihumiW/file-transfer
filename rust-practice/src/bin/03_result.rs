fn main(){
    // Result 类型用于表示可能成功或失败的操作
    let config = read_config();
    match config{
        Ok(content) => println!("Config content: {}", content),
        Err(e) => println!("Error reading config: {}", e),
    }
}

fn read_config() -> Result<String, String> {
    // 模拟读取配置文件的操作
    let config_content = "config data"; // 假设这是从文件中读取到的内容
    if config_content.is_empty() {
        Err("Failed to read config".to_string())
    } else {
        Ok(config_content.to_string())
    }

}

