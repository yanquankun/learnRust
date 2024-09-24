fn main() {
    // 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 通过&符号来引用s1的值，允许你使用值但不获取其所有权
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    let mut s2: String = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s;
    println!("{}", r3);
}
fn calculate_length(s: &String) -> usize {
    // s 是 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
