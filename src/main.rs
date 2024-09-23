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

// 3.3 函数
// fn main() {
//     // 语句（Statements）是执行一些操作但不返回值的指令
//     // 表达式（Expressions）计算并产生一个值

//     // 这种写法是错误的，rust和其他语言不同，let y = 5 并不会产生返回值
//     // let x = (let y = 5); ❌

//     let y = {
//         let x = 3;
//         // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
//         x + 1
//     };
//     println!("The value of y is: {y}");

//     // 所有函数名采用snake_case风格
//     // 如果非要驼峰也可以运行，但不符合规范
//     another_function(10, 666);

//     let five = five();
//     println!("The value of five is: {five}");
// }

// fn five() -> i32 {
//     // 采用return或者无分号结尾都可以形成一个表达式
//     // return 5;
//     // or
//     5
// }

// fn another_function(x: i8, y: i32) {
//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
// }

// 3.4 注释
// fn main() {
//     //! # 文档注释
//     //! @link https://www.baidu.com
//     //! @author Mint.Yan
//     //! A library for modeling artistic concepts.

//     // 单行注释
// }

// 3.5 控制流
// fn main() {
//     let number = 15;

//     // rust if表达式必须是一个布尔表达式
//     // rust 本身不会帮你做隐式转换
//     if number < 5 {
//         println!("condition was true");
//     } else if number == 5 {
//         println!("condition was 5");
//     } else {
//         println!("condition was false");
//     }

//     let condition = true;
//     // 通过if表达式返回值时，必须保证每个分支的值是相同的类型
//     // 因为rust会自动推导表达式左侧的类型
//     let number = if condition { 1 } else { 0 };

//     println!("The value of number is: {number}");

//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The value of result is: {result}");

//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");

//     let mut number = 0;
//     while number <= 3 {
//         println!("number = {number}");

//         number += 1;
//     }

//     let mut arr = [1, 2, 3, 4, 5];
//     for (index, element) in arr.iter().enumerate() {
//         println!("the value is: {element}, at position: {index}");
//     }
//     arr.reverse();
//     for element in arr {
//         println!("the value is: {element}");
//     }

//     // 1..4 代表的是从[start]到[end-1]位置的值，实际上只有1 2 3
//     // 所以下面两种for循环会输出1 2 3的值
//     for number in 1..4 {
//         println!("{number}!");
//     }
//     println!("------------------");
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
// }

// 4.1 所有权
// fn main() {
//     // 所有权机制 内存管理机制
//     // 1. 每个值都有一个所有者
//     // 2. 值在所有者离开作用域时被释放
//     // 3. 当值的所有者离开作用域时，该值将被删除

//     {
//         // s 在这里无效，它尚未声明
//         let _s = "hello"; // 从此处起，s 是有效的

//         // 使用 s
//     } // 此作用域已结束，s 不再有效
//       // rust自动在 } 处释放 s 的内存，会自动调用drop函数

//     let s: String = String::from("我是不可修改的字面值");
//     println!("{}", s);

//     // String 类型 分配在堆上  适合位置长度的数据
//     let mut s = String::from("hello我是可修改的String");
//     s.push_str(", world!"); // push_str() 在字符串后追加文字
//     println!("{} len is {}", s, s.len()); // 将打印 `hello, world!`

//     // 需要定义 Object 结构体
//     struct Object {
//         name: String,
//         age: u32,
//     }
//     let obj: Object = Object {
//         name: String::from("Mint.Yan"),
//         age: 18,
//     };
//     println!("name is {}, age is {}", obj.name, obj.age);

//     let s1 = String::from("hello");
//     let s2 = s1; // 这里s2拷贝了s1，同时也对s1进行了释放，此时已经无法访问s1了
//     println!("{}", s2);

//     // 深拷贝
//     // rust 永远不会自动创建深拷贝，因为深拷贝会占用大量资源
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);

//     // 函数所有权
//     let s = String::from("hello"); // s 进入作用域
//     takes_ownership(s); // s 的值移动到函数里
//                         //所以到这里不再有效
//     let x = 5; // x 进入作用域
//     makes_copy(x); // x 应该移动函数里，
//                    // 但 i32 是 Copy 的，
//                    // 所以在后面可继续使用 x

//     let sss = gives_ownership();
//     println!("{}", sss);

//     let (str, len): (String, usize) = calculate_length(String::from("hello"));
//     println!("The length of '{}' is {}.", str, len);
// } // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。
//   // 占用的内存被释放

// fn makes_copy(some_integer: i32) {
//     // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。没有特殊之处

// fn gives_ownership() -> String {
//     // gives_ownership 会将
//     // 返回值移动给
//     // 调用它的函数

//     let some_string = String::from("yours"); // some_string 进入作用域。

//     some_string
//     // return some_string; // 等同直接写some_string 注意：不要写分号!!!
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度

//     (s, length)
// }
