fn main() {
    let mut s = String::new();

    let str: &str = "这是一个字符串";

    let s = str.to_string();
    println!("{}", s);

    let s = "这是一个字符串".to_string();
    println!("{}", s);

    let mut s = String::from("这是一个字符串");
    println!("{}", s);

    // 更新字符串
    s.push_str(" 哈哈");
    println!("{}", s);

    s.push('哈');
    println!("{}", s);

    let one = String::from("1");
    let two = String::from("2");

    // 此时 one 变量已经不能再用了，因为已经被add函数使用
    let add = one + "+" + &two;
    println!("{}", add);

    let three = String::from("3");
    let add = format!("1+{two}+{three}");
    println!("{}", add);

    // rust中要按照字节数获取部分字符串
    let s = String::from("你好");
    let piece = &s[0..3];
    println!("{}", piece);

    // 遍历
    // chars返回原始字符
    for str in s.chars() {
        println!("{}", str);
    }
    // bytes返回原始字节
    for str in s.bytes() {
        println!("{}", str);
    }
}
