/**
 * 2024-09-18 18:39:03
 * @author Mint.Yan
 * @description 学习 Rust Good Good Study Day Day Up
 * @link https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html 
*/

// 2.猜数字游戏
// use rand::Rng;
// use std::{cmp::Ordering, io};
// fn main() {
//     println!("猜数字游戏");

//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("秘密数字是: {secret_number}");

//     loop {
//         println!("请输入你猜的数字:");

//         // let guess = 5; // 不可变
//         let mut guess: String = String::new(); // 可变

//         io::stdin()
//             .read_line(&mut guess) // 返回一个 result
//             .expect("Failed to read line"); // expect 会把错误信息打印出来，然后退出程序，等于是 cathe 一样的效果

//         // expect 异常捕获模式
//         // let guess: u32 = guess.trim().parse().expect("请输入一个数字");
//         // continue 模式
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("请输入一个数字");
//                 continue;
//             }
//         };

//         println!("你猜的数字是: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("猜小了"),
//             Ordering::Equal => {
//                 println!("猜对了");
//                 break;
//             }
//             Ordering::Greater => println!("猜大了"),
//         }
//     }
// }

// 3.1变量
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
//     // 变量重写
//     let x = x * 2;
//     println!("The value of x is: {x}");
//     // 常量必须使用 UPPER_CAMEL+_ 命名
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
//     let spaces = "    ";
//     let spaces = spaces.len();
//     println!("The value of spaces is: {spaces}");
// }

// use std::num::Wrapping;
// // 3.2数据类型
// fn main() {
//     // 整型、浮点型、布尔类型和字符类型

//     // 整型
//     // 长度	    有符号	 无符号
//     // 8-bit	i8	    u8
//     // 16-bit	i16	    u16
//     // 32-bit	i32	    u32
//     // 64-bit	i64	    u64
//     // 128-bit	i128    u128
//     // arch	    isize	usize
//     let x: Wrapping<i8> = Wrapping(100);
//     println!("The value of x is: {x}");

//     // 浮点型
//     // f32 是单精度浮点数
//     // f64 是双精度浮点数。
//     let y: f32 = 3.0123213;
//     println!("The value of y is: {y}");
//     let z: f64 = 3.111;
//     println!("The value of z is: {z}");

//     let remainder = 43 % 5;
//     println!("The value of remainder is: {remainder}");

//     // 布尔类型
//     let t = true;
//     let f: bool = false;
//     println!("The value of t is: {t}");
//     println!("The value of f is: {f}");
//     let _bol = true;
//     let mut _bol = false;
//     let _bol2: u32 = _bol as u32;
//     let _bol3: u32 = true as u32;
//     println!("The value of bol is: {_bol}");
//     println!("The value of bol is: {_bol2}");
//     println!("The value of bol is: {_bol3}");

//     // 字符类型
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
//     println!("{c} {z} {heart_eyed_cat}");

//     // 元组
//     let tup: (i64, u32, char) = (-20, 100, '😻');
//     let (a, b, c) = tup;
//     let d: char = tup.2;
//     println!("{a} {b} {c} {d}");

//     // 数组
//     let a: [char; 3] = ['1', '2', '3'];
//     let a1 = a[1];
//     println!("{a:?} {a1}");
// }

// 3.3函数
fn main() {}
