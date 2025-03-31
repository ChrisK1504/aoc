use std::cmp;
use std::env;
use std::fs;

pub fn solve(input: String) -> (u32, u32) {
    let mut total_paper: u32 = 0;
    let mut total_ribbon: u32 = 0;
    for line in input.lines() {
        let mut dimensions: Vec<u32> = line
            .split('x')
            .map(|s: &str| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        dimensions.sort_unstable();

        if dimensions.len() != 3 {
            continue;
        }

        let l: u32 = dimensions[0];
        let w: u32 = dimensions[1];
        let h: u32 = dimensions[2];

        let wrapping_paper: u32 = 2 * l * w + 2 * w * h + 2 * h * l;
        let slack: u32 = cmp::min(l * w, cmp::min(l * h, w * h));

        total_paper += wrapping_paper + slack;

        let bow_tie: u32 = l * w * h;
        let ribbon: u32 = cmp::min(2 * (l + w), cmp::min(2 * (l + h), 2 * (w + h)));

        total_ribbon += ribbon + bow_tie;
    }

    return (total_paper, total_ribbon);
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path: &String = &args[1];
    let mut input: String = fs::read_to_string(file_path).expect("Couldn't read the file");

    input = input.trim().to_string();
    let (paper, ribbon) = solve(input);
    println!("Paper: {}; Ribbon: {}", paper, ribbon);
}
