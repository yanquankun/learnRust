struct Score<T> {
    x: T,
    y: T,
}
// 通过对泛型设置T U支持该Point类型可设置不同类型的值
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let _score: Score<i32> = Score { x: 5, y: 6 };
    let _point: Point<i32, char> = Point { x: 5, y: '6' };

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result: &i32 = largest(&number_list);
    println!("The largest number is {result}");

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];

    let result: &char = largest(&char_list);
    println!("The largest char is {result}");
}
// T需要实现PartialOrd的Trait，否则无法比较
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
