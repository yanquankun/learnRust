fn main() {
    // 语句（Statements）是执行一些操作但不返回值的指令
    // 表达式（Expressions）计算并产生一个值

    // 这种写法是错误的，rust和其他语言不同，let y = 5 并不会产生返回值
    // let x = (let y = 5); ❌

    let y = {
        let x = 3;
        // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
        x + 1
    };
    println!("The value of y is: {y}");

    // 所有函数名采用snake_case风格
    // 如果非要驼峰也可以运行，但不符合规范
    another_function(10, 666);

    let five = five();
    println!("The value of five is: {five}");
}

fn five() -> i32 {
    // 采用return或者无分号结尾都可以形成一个表达式
    // return 5;
    // or
    5
}

fn another_function(x: i8, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}