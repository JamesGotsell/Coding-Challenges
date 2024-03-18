use std::env;
use std::fs;
fn main() {
    println!("Hello, world!");
    let mut args: Vec<_> = env::args().collect();
    let stuff = args;
    let file_path = stuff[1];

    println!("In file {:?}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
