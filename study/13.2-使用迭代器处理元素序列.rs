fn main() {
    use tests::filters_by_size;

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val);
    }

    let mut v2_iter = v1.iter();
    // assert_eq! 宏用于测试相等性并在不相等时引发错误
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    let sum: i32 = v1.iter().sum();
    println!("sum is {}", sum);

    let v2 = vec![1, 2, 3];
    let v2_increase: Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!("{:?}", v2_increase);

    filters_by_size();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 使用 filter 方法来获取一个闭包。该闭包从迭代器中获取一项并返回一个 bool。如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会被包含。
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

mod tests {
    use super::*;

    pub fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("nike"),
            },
            Shoe {
                size: 12,
                style: String::from("adidas"),
            },
            Shoe {
                size: 12,
                style: String::from("puma"),
            },
        ];

        let my_shoes = shoes_in_size(shoes, 12);

        println!("{:?}", my_shoes);
    }
}
