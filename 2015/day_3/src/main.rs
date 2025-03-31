use rustc_hash::FxHashSet;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    let data = std::fs::read_to_string(file_path).expect("Couldn't load file");
    let (santa, robo_santa) = solve(&data);
    println!("Santa: {}\n Robo+Santa: {}", santa, robo_santa);
}

fn solve(data: &str) -> (usize, usize) {
    return (solve1(data), solve2(data));
}

pub fn solve1(data: &str) -> usize {
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

fn solve2(data: &str) -> usize {
    let mut visited = FxHashSet::default();

    let mut position_santa = (0, 0);
    let mut position_robot = (0, 0);

    visited.insert(position_santa);

    for (i, dir) in data.chars().enumerate() {
        if i % 2 == 0 {
            match dir {
                '<' => position_santa.0 -= 1,
                '>' => position_santa.0 += 1,
                'v' => position_santa.1 -= 1,
                '^' => position_santa.1 += 1,
                _ => panic!("invalid direction at {dir}"),
            }

            visited.insert(position_santa);
        } else {
            match dir {
                '<' => position_robot.0 -= 1,
                '>' => position_robot.0 += 1,
                'v' => position_robot.1 -= 1,
                '^' => position_robot.1 += 1,
                _ => panic!("invalid direction at {dir}"),
            }

            visited.insert(position_robot);
        }
    }

    return visited.len();
}
