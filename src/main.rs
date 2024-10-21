/// 日期: 2024-09-18 18:39:03
/// 作者: Mint.Yan
/// 描述: 学习 Rust 的好方法，努力学习，天天向上
/// 链接: [Rust 学习链接](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html)
///

// Vector只能储存相同类型的值
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);
}
