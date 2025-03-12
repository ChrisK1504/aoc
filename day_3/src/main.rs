use rustc_hash::FxHashSet;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    let data = std::fs::read_to_string(file_path).expect("Couldn't load file");
    println!("{}", solve(&data));
}

pub fn solve(data: &str) -> usize {
    let mut visited = FxHashSet::default();

    let mut position = (0, 0);

    visited.insert(position);

    for dir in data.chars() {
        match dir {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '^' => position.1 += 1,
            _ => panic!("invalid direction at {dir}"),
        }

        visited.insert(position);
    }

    return visited.len();
}
