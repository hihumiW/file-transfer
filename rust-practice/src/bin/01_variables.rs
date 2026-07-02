fn main(){
    //变量声明
    let x : i32 = 5; // rust 声明的变量默认不可变
    let mut y : i32 = 10; // 需要使用 mut 关键字声明可变变量
    // y = false; Error! 变量类型不匹配
    y = add(y , 10); // 可变变量可以被修改
    println!("变量 x 的值为: {}, 变量 y 的值为: {}", x, y); // 这里的 println! 是宏， 并非是普通函数

    let _a_bool : bool = true; // 布尔类型
    let a_float : f32 = 0.1; // 浮点类型
    println!("0.1 + 0.2 = {}", a_float + 0.2); // 试试会不会精度丢失
 
    // 字符串类型
    let a_string : &str = "Hello, Rust!"; // 字符串切片类型
    let mut a_string2 : String = String::from(a_string); 
    a_string2.push_str(" I am a mutable String."); // 可变字符串可以被修改
    println!("a_string: {}, a_string2: {}", a_string, a_string2);

    //Vec 列表
    let mut a_vector : Vec<i32> = Vec::new();
    a_vector.push(1); // 向 Vec 中添加元素
    a_vector.push(2);
    let mut a_vector2 : Vec<i32> = vec![1, 2, 3, 4, 5]; // 或者使用宏，快速方便的初始化
    a_vector2.push(6);
    println!("a_vector2 {a_vector2:?}");

    let a_verctor3 : Vec<i32> = Vec::with_capacity(10); //创建一个容量为10的Vec, 但是长度为0
    assert_eq!(a_verctor3.capacity(), 10); // 容量为10
    assert_eq!(a_verctor3.len(), 0); // 长度为0
    
}

fn add(x : i32 , y : i32) -> i32{
    x + y //最后一行没有; 标识返回值   
}