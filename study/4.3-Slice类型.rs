fn main() {
    let s: String = String::from("hello world");
    println!("{}", s);
    // 类比js的slice用法
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{} {}", hello, world);
    let all: &str = &s[..];
    println!("{}", all);
    let word: &str = first_word(&s);
    println!("{}", word);

    let str: &str = "i am boy";
    let i: &str = first_word(&str[0..1]);
    println!("{}", i);
    let am: &str = first_word(&str[2..]);
    println!("{}", am);
    let word = first_word(str);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:#?}", slice);
    assert_eq!(slice, &[2, 3]);
    println!("{:?}", slice);
}

// 采用&str而不是&String
// 在函数 first_word 中，使用 &str 类型而不是 &String 类型。原因是 &str 是不可变的引用，表示对字符串切片的只读访问，而 &String 是可变的引用，表示对整个字符串的可变访问。在这个上下文中，使用不可变的 &str 更合适，因为我们只需要读取字符串的一部分，而不需要修改它。这样可以提高代码的安全性和性能。
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}