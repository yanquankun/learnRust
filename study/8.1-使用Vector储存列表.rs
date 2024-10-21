// Vector只能储存相同类型的值
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    // 语法糖 vec! 宏，这个宏会根据我们提供的值来创建一个新的 vector
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // 读取Vector中的元素
    let v3 = vec![1, 2, 3, 4, 5];

    // 通过下界获取元素
    // 无法获取超过边界的元素，会报错
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    // 通过get获取元素
    // 可以超过vec边界，返回None
    let one: Option<&i32> = v3.get(1);
    match one {
        Some(one) => println!("The third element is {}", one),
        None => println!("There is no third element."),
    }

    // 循环
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    // 储存不同类型的值
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v5: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", v5);
}
