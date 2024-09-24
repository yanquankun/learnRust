use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("猜数字游戏");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("秘密数字是: {secret_number}");

    loop {
        println!("请输入你猜的数字:");

        // let guess = 5; // 不可变
        let mut guess: String = String::new(); // 可变

        io::stdin()
            .read_line(&mut guess) // 返回一个 result
            .expect("Failed to read line"); // expect 会把错误信息打印出来，然后退出程序，等于是 cathe 一样的效果

        // expect 异常捕获模式
        // let guess: u32 = guess.trim().parse().expect("请输入一个数字");
        // continue 模式
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字");
                continue;
            }
        };

        println!("你猜的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
            Ordering::Greater => println!("猜大了"),
        }
    }
}
