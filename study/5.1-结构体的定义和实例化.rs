use std::fmt;
struct User {
    name: String,
    age: u8,
    phone: String,
    email: String,
}

// 实现Display特征
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fields = vec![
            format!("Name: {}", self.name),
            format!("Age: {}", self.age),
            format!("Phone: {}", self.phone),
            format!("Email: {}", self.email),
        ];
        write!(f, "{}", fields.join(", "))
    }
}
fn main() {
    let mut user1: User = User {
        name: String::from("Mint.Yan"),
        age: 20,
        phone: String::from("13111111111"),
        email: String::from("123456789@qq.com"),
    };

    println!("{}", user1);

    user1.age = 30;
    println!("{}", user1);

    let new_user: User = create_user(String::from("Mint.Yan2"), 31);
    println!("{}", new_user);

    let cp_new_user: User = User {
        name: String::from("Mint.Yan.cp"),
        // 使用 ..new_user 来复制 new_user 中的所有字段
        // 此处和js的展开语法糖不同，Rust 中使用 .. 来表示结构体中的所有字段
        // ..复制必须放到最后
        ..new_user
    };
    println!("{}", cp_new_user);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(12, 0, 0);
    println!("black = {}, origin = {}", black.0, origin.0);
}
fn create_user(name: String, age: u8) -> User {
    User {
        name,
        age,
        phone: String::from("13111111111"),
        email: String::from("123456789@qq.com"),
    }
}