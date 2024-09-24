fn main() {
    // 所有权机制 内存管理机制
    // 1. 每个值都有一个所有者
    // 2. 值在所有者离开作用域时被释放
    // 3. 当值的所有者离开作用域时，该值将被删除

    {
        // s 在这里无效，它尚未声明
        let _s = "hello"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s 不再有效
      // rust自动在 } 处释放 s 的内存，会自动调用drop函数

    let s: String = String::from("我是不可修改的字面值");
    println!("{}", s);

    // String 类型 分配在堆上  适合位置长度的数据
    let mut s = String::from("hello我是可修改的String");
    s.push_str(", world!"); // push_str() 在字符串后追加文字
    println!("{} len is {}", s, s.len()); // 将打印 `hello, world!`

    // 需要定义 Object 结构体
    struct Object {
        name: String,
        age: u32,
    }
    let obj: Object = Object {
        name: String::from("Mint.Yan"),
        age: 18,
    };
    println!("name is {}, age is {}", obj.name, obj.age);

    let s1 = String::from("hello");
    let s2 = s1; // 这里s2拷贝了s1，同时也对s1进行了释放，此时已经无法访问s1了
    println!("{}", s2);

    // 深拷贝
    // rust 永远不会自动创建深拷贝，因为深拷贝会占用大量资源
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 函数所有权
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里
                        //所以到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x

    let sss = gives_ownership();
    println!("{}", sss);

    let (str, len): (String, usize) = calculate_length(String::from("hello"));
    println!("The length of '{}' is {}.", str, len);
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string
    // return some_string; // 等同直接写some_string 注意：不要写分号!!!
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}