use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut input = fs::read_to_string(file_path).expect("Couldn't read the file");
    input = input.trim().to_string();

    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    println!("Floor: {floor}");
}
