use std::path::PathBuf;

fn main() {
    p1();
    p2();
    p3();
    p4();
    p5();
    p6();
}

fn format_device_name(name: &str) -> String {
    // let mut result = String::from("Device: ");
    // result.push_str(name);
    // result

    //
    format!("Device: {}", name)
}

#[allow(dead_code)]
#[derive(Debug)]
struct DeviceInfo {
    device_name: String,
    device_id: String,
    receive_enabled: bool,
}

fn p1() {
    let device_name = format_device_name("Thinkbook");
    println!("{}", device_name);

    let device_info = create_device_info();
    println!("Device Info: {device_info:?}");
}

//--------------------

fn create_device_info() -> DeviceInfo {
    let device = DeviceInfo {
        device_name: "Thinkbook".to_string(),
        device_id: "demo-device-id".to_string(),
        receive_enabled: true,
    };
    device
}

fn get_display_name(custom_name: Option<&str>, system_name: &str) -> String {
    if let Some(name) = custom_name {
        name.to_string()
    } else {
        system_name.to_string()
    }
}

fn p2() {
    let custom_name: Option<String> = Some("My PC".to_string());
    let system_name = "DESKTOP-123".to_string();

    // as_deref() 会将 Option<String> 转化为 Option<&str>
    println!(
        "Display Name: {}",
        get_display_name(custom_name.as_deref(), &system_name)
    );
    println!("Display Name: {}", get_display_name(None, &system_name));
}

//--------------------

// 7788-7888 返回Ok(port), 其他返回Err
fn validate_port(port: u16) -> Result<u16, String> {
    if port >= 7788 && port <= 7888 {
        Ok(port)
    } else {
        Err(format!(
            "Invalid port: {}. Port must be between 7788 and 7888.",
            port
        ))
    }
}

fn p3() {
    println!("Validating port 7800: {:?}", validate_port(7800));
    println!("Validating port 7900: {:?}", validate_port(7900));
}

//--------------------

#[allow(dead_code)]
#[derive(Debug)]
struct FileInfo {
    name: String,
    size: u64,
}

// 接受一个 切片引用（无论是元组，还是Vec）
#[allow(unused_doc_comments)]
fn total_size(files: &[FileInfo]) -> u64 {
    // let mut total : u64 = 0;
    // for file in files  {
    //     total += file.size;
    // }
    // total

    /*
     在rust中，Vec本身只是一个集合容器， 没有迭代的功能, 需要用户手动选择迭代器
     files.iter() 借用遍历 得到 &FileInfo
     files.into_iter() 消耗遍历， 会获取所有权 得到 FileInfo
     files.iter_mut() 可变借用遍历 得到  &mut FileInfo
    */
    files.iter().map(|x| x.size).sum()
}

fn p4() {
    let files = vec![
        FileInfo {
            name: "file1.txt".to_string(),
            size: 1024,
        },
        FileInfo {
            name: "file2.txt".to_string(),
            size: 2048,
        },
        FileInfo {
            name: "file3.txt".to_string(),
            size: 512,
        },
    ];
    println!("Total size of files: {}", total_size(&files));
}

//--------------------

#[allow(dead_code)]
enum TransferStatus {
    Pending,
    Accepted,
    Rejected,
    Uploading,
    Completed,
    Failed,
}

fn status_text(status: TransferStatus) -> &'static str {
    match status {
        TransferStatus::Pending => "等待确认",
        TransferStatus::Accepted => "已接受",
        TransferStatus::Rejected => "已拒绝",
        TransferStatus::Uploading => "上传中",
        TransferStatus::Completed => "已完成",
        TransferStatus::Failed => "失败",
    }
}

fn p5() {
    println!(
        "Status text for TransferStatus::Pending: {}",
        status_text(TransferStatus::Pending)
    );
}

//--------------------

fn build_save_path(download_dir: PathBuf, file_name: &str) -> PathBuf {
    let mut path = download_dir;
    path.push("LanTransfer");
    path.push(file_name);
    path
}

fn p6() {
    println!(
        "Save path: {:?}",
        build_save_path(PathBuf::from("D:\\Downloads"), "file.txt")
    );
}

//--------------------

