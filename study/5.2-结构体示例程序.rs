#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_rectangle_area(width, height)
    );

    // 使用元祖重构宽高
    let rect: (f32, f32) = (30.66, 50.66);
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_rectangle_area_tuple(rect)
    );
    // 上述模式在使用时，需要知道元祖的顺序，如果顺序不对，则会出现未知错误

    // 使用结构体重构宽高
    let rect: Rectangle = Rectangle {
        width: 50,
        height: 60,
    };
    println!("rect is {:#?}", rect);
    dbg!(&rect);
    // [src/main.rs:33:5] &rect = Rectangle {
    //     width: 50,
    //     height: 60,
    // }
    println!(
        "The area of the rectangle is {} square pixels.",
        calc_rectangle_area_struct(rect)
    );
}
// 计算长方体面积
fn calc_rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}
fn calc_rectangle_area_tuple(dimensions: (f32, f32)) -> f32 {
    dimensions.0 * dimensions.1
}
fn calc_rectangle_area_struct(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}