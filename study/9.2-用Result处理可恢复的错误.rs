use std::fs::File;
use std::io::{self, Read};
fn main() {
    // File::open("hello.txt").expect("123");

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("./hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    let username = read_username_from_file();
    println!("{:#?}", username);

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    let str = last_char_of_first_line("123\n456");
    println!("{:#?}", str);

    // 当 main 函数返回 Result<(), E>，如果 main 返回 Ok(()) 可执行程序会以 0 值退出，而如果 main 返回 Err 值则会以非零值退出；成功退出的程序会返回整数 0，运行错误的程序会返回非 0 的整数。Rust 也会从二进制程序中返回与这个惯例相兼容的整数。

    // panic类似js中的throw，抛出异常
}
