fn main() {
    // cmd cargo run > output.txt
    // 将输出到文件中
    print!("Hello, world!");
    // output.txt: Hello, world!

    let err: String = String::from("error");
    // eprintln! 将错误信息写入标准错误而不是标准输出
    // 此错误将作为标准错误输出到控制台
    eprintln!("Problem parsing arguments: {err}");
}
