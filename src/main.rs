use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file {file_path}");
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("Width text:\n{content}");
}
