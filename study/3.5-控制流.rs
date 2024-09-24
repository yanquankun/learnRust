fn main() {
    let number = 15;

    // rust if表达式必须是一个布尔表达式
    // rust 本身不会帮你做隐式转换
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was 5");
    } else {
        println!("condition was false");
    }

    let condition = true;
    // 通过if表达式返回值时，必须保证每个分支的值是相同的类型
    // 因为rust会自动推导表达式左侧的类型
    let number = if condition { 1 } else { 0 };

    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 0;
    while number <= 3 {
        println!("number = {number}");

        number += 1;
    }

    let mut arr = [1, 2, 3, 4, 5];
    for (index, element) in arr.iter().enumerate() {
        println!("the value is: {element}, at position: {index}");
    }
    arr.reverse();
    for element in arr {
        println!("the value is: {element}");
    }

    // 1..4 代表的是从[start]到[end-1]位置的值，实际上只有1 2 3
    // 所以下面两种for循环会输出1 2 3的值
    for number in 1..4 {
        println!("{number}!");
    }
    println!("------------------");
    for number in (1..4).rev() {
        println!("{number}!");
    }
}