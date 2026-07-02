fn main(){
    let name1 : Option<String> = Some("Thinkbook".to_string());
    let name2 : Option<String> = None;

    //  & 代表借用 name1 的所有权, 这样就不会发生所有权转移, name1 仍然可以继续使用
    match &name1 {
        Some(value) => println!("name1 is: {value}"),
        None => println!("name1 is None"),
    }
    // as_ref() 方法将 Option<T> 转换为 Option<&T>，这样就可以借用内部的值而不获取所有权
    if let Some(value) = name2.as_ref() {
        println!("name2 is: {value}");
    } else {
        println!("name2 is None");
    }

    let display_name = name2.unwrap_or("Default Name".to_string());
    println!("display_name is: {display_name}");
    println!("name1 is: {:?}", name1);
    
}