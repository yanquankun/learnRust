use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // print!("{:?}", args);
    dbg!(&args);

    // cmd cargo run -- arg1 1 arg2 2
    let arg1 = &args[1];
    println!("{}", arg1);
    let val1 = &args[2];
    println!("{}", val1);

    let arg2 = &args[3];
    println!("{}", arg2);
    let val2 = &args[4];
    println!("{}", val2);
}
