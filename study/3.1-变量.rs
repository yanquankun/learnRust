fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // 变量重写
    let x = x * 2;
    println!("The value of x is: {x}");
    // 常量必须使用 UPPER_CAMEL+_ 命名
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}