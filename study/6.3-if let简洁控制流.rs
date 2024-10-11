enum Coin {
    Dime,
    Quarter,
}
fn main() {
    let num_max = Some(3u8);
    // if let是match的语法糖
    if let Some(max) = num_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count: i32 = 0;
    let coin: Coin = Coin::Quarter;
    if let Coin::Quarter = coin {
        println!("if condition count: {}", count);
    } else {
        count += 1;
        println!("else condition count: {}", count);
    }
}
