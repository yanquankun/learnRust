#[derive(Debug)]
enum IpAddrEnum {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAndAddrEnum {
    V4Adr(String),
    V6Adr(String),
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrEnum,
    address: String,
}
#[derive(Debug)]
enum Message {
    Write(String),
}
impl Message {
    fn call(&self) {
        println!("call {:?}", &self);
    }
}
fn main() {
    let ipv4: IpAddrEnum = IpAddrEnum::V4;
    let ipv6: IpAddrEnum = IpAddrEnum::V6;
    println!("ipv4: {:#?}, ipv6: {:#?}", ipv4, ipv6);
    let home: IpAddr = IpAddr {
        kind: ipv4,
        address: String::from("127.0.0.1"),
    };
    println!("home: {:#?}", home);
    println!(
        "home.kind: {:#?}, home.address: {:#?}",
        home.kind, home.address
    );
    let home: IpAndAddrEnum = IpAndAddrEnum::V4Adr(String::from("127.0.0.1"));
    println!("home: {:#?}", home);
    let home: IpAndAddrEnum = IpAndAddrEnum::V6Adr(String::from("::1"));
    println!("home: {:#?}", home);

    let ins: Message = Message::Write(String::from("hello"));
    ins.call();

    // Option 枚举 rust 中没有 null 类型，只有 Option 枚举类型，内置在标准库中
    // enum Option<T> {
    //   None, // 没有值
    //   Some(T), // 有一个值
    // }
    let some_number: Option<i32> = Some(5);
    let some_string: Option<String> = Some(String::from("a string"));
    let absent_number: Option<i128> = None;
    println!(
        " some_string: {:#?}, absent_number: {:#?}",
        some_string, absent_number
    );
    if some_number != None {
        println!("some_number: {:#?}", some_number);
    }
}
