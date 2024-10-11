enum Coin {
    // Penny,
    // Nickel,
    // Dime,
    Quarter,
}
fn main() {
    let coin: Coin = Coin::Quarter;
    let value: i32 = value_in_cents(coin);
    println!("value: {:?}", value);

    let none = plue_one(None);
    println!("none: {:?}", none);
    let five = plue_one(Some(4));
    println!("five: {:?}", five);

    // 通配模式和_占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 你可以理解为switch的default，必须放到最后
        _ => move_player(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player() {}
}
/// 对于match 你可以理解为js中的switch
fn value_in_cents(coin: Coin) -> i32 {
    let value: i32 = match coin {
        // Coin::Penny => 1,
        // Coin::Nickel => 5,
        // Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    return value;
}
/// 对Option<T> 你可以理解为typescript中的Option<T>
fn plue_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // 对于Some，你可以理解为typescript中的Pick<T>
        Some(i) => Some(i + 1),
    }
}