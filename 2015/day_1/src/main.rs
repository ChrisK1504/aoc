use std::env;
use std::fs;

pub fn solve(input: String) -> (i32, i32) {
    let mut floor = 0;
    let mut basement = 0;
    let mut position = 0;
    for c in input.chars() {
        position += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if floor == -1 && basement == 0 {
            basement = position;
        }
    }

    return (floor, basement);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut input = fs::read_to_string(file_path).expect("Couldn't read the file");
    input = input.trim().to_string();

    let (floor, basement) = solve(input);
    println!("Floor: {}\n Basement: {}", floor, basement);
}
