use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    print!("读取中，请耐心等待...\n");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Contents:\n{}", contents);
}
