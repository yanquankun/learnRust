use std::num::Wrapping;
fn main() {
    // 整型、浮点型、布尔类型和字符类型

    // 整型
    // 长度	    有符号	 无符号
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128    u128
    // arch	    isize	usize
    let x: Wrapping<i8> = Wrapping(100);
    println!("The value of x is: {x}");

    // 浮点型
    // f32 是单精度浮点数
    // f64 是双精度浮点数。
    let y: f32 = 3.0123213;
    println!("The value of y is: {y}");
    let z: f64 = 3.111;
    println!("The value of z is: {z}");

    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // 布尔类型
    let t = true;
    let f: bool = false;
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");
    let _bol = true;
    let mut _bol = false;
    let _bol2: u32 = _bol as u32;
    let _bol3: u32 = true as u32;
    println!("The value of bol is: {_bol}");
    println!("The value of bol is: {_bol2}");
    println!("The value of bol is: {_bol3}");

    // 字符类型
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // 元组
    let tup: (i64, u32, char) = (-20, 100, '😻');
    let (a, b, c) = tup;
    let d: char = tup.2;
    println!("{a} {b} {c} {d}");

    // 数组
    let a: [char; 3] = ['1', '2', '3'];
    let a1 = a[1];
    println!("{a:?} {a1}");
}
