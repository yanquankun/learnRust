use std::collections::HashMap;
fn main() {
    // hashmap的键必须都是同类型 值也必须都是同类型
    let mut map: HashMap<String, &str> = HashMap::new();
    map.insert(String::from("name"), "yqk");
    map.insert(String::from("age"), "30");

    println!("{:?}", map);

    let age = String::from("age");
    let age = map.get(&age);
    println!("{:?}", age);

    let name = String::from("name");
    // copied 方法来获取一个 String 而不是 &String
    // unwrap_or 默认值
    let name = map.get(&name).copied().unwrap_or("unknow");
    println!("{:?}", name);

    // 循环
    for (k, v) in &map {
        println!("{k}:{v}");
    }

    // 检查是否存在
    if let Some(v) = &map.get(&String::from("name")) {
        println!("{v}");
    } else {
        println!("不存在");
    }

    // 检查是否存在并插入默认值
    // entry检测值是否存在，不存在就插入，存在就返回旧值
    let mut scores = HashMap::new();
    scores.entry("a").or_insert(100);
    scores.entry("b").or_insert(200);
    println!("{scores:?}");

    // 更新旧值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
