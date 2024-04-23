use std::fs;

fn main() {
    println!("Hello, world!");
    match fs::read_to_string("a") {
        Ok(contents) => println!("'a' contents:\n{contents}"),
        Err(e) => println!("reading 'a' error: {e}"),
    };
    match fs::read_to_string("b") {
        Ok(contents) => println!("'b' contents:\n{contents}"),
        Err(e) => println!("reading 'b' error: {e}"),
    };
}
