use std::cmp;
use std::env;
use std::fs;

pub fn solve(input: String) -> u32 {
    let mut total_paper = 0;
    for line in input.lines() {
        let mut dimensions = line
            .split('x')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        dimensions.sort_unstable();

        if dimensions.len() != 3 {
            continue;
        }

        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];

        let wrapping_paper = 2 * l * w + 2 * w * h + 2 * h * l;

        let slack = cmp::min(l * w, cmp::min(l * h, w * h));

        total_paper += wrapping_paper + slack;
    }

    return total_paper;
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let mut input = fs::read_to_string(file_path).expect("Couldn't read the file");

    input = input.trim().to_string();
    let answer = solve(input);
    println!("{answer}");
}
