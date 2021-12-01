use std::fs::read_to_string;
use std::collections::BTreeSet;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_instructions = parse_instructions(&input_string);
    let black_tiles = follow_steps(&parsed_instructions);
    println!("1: {}", black_tiles.len());
    let black_tiles_after_100_days = execute_daily_flips(100,&black_tiles);
    println!("2: {}", black_tiles_after_100_days.len());
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Step {
    EAST,
    SOUTHEAST,
    NORTHEAST,
    WEST,
    SOUTHWEST,
    NORTHWEST
}

impl Step {
    fn get_coords(&self) -> (i32, i32) {
        match self {
            Step::EAST => (2, 0),
            Step::SOUTHEAST => (1, -1),
            Step::NORTHEAST => (1, 1),
            Step::WEST => (-2, 0),
            Step::SOUTHWEST => (-1, -1),
            Step::NORTHWEST => (-1, 1),
        }
    }

    fn values() -> [Step; 6] {
        [Step::EAST, Step::SOUTHEAST, Step::NORTHEAST, Step::WEST, Step::SOUTHWEST, Step::NORTHWEST]
    }
}

fn parse_instructions(input: &str) -> Vec<Vec<Step>> {
    input.split("\n")
        .filter(|e| !e.is_empty())
        .map(|e| parse_instruction_line(e))
        .collect()
}

fn parse_instruction_line(input: &str) -> Vec<Step> {
    let mut vec: Vec<Step> = Vec::new();
    let mut char_iter = input.chars();
    while let Some(c) = char_iter.next() {
        let next_step = match c {
            'w' => Step::WEST,
            'e' => Step::EAST,
            's' => {
                let next_char = char_iter.next().unwrap();
                match next_char {
                    'e' => Step::SOUTHEAST,
                    'w' => Step::SOUTHWEST,
                    _ => panic!("char {} not expected after 's'", next_char)
                }
            },
            'n' => {
                let next_char = char_iter.next().unwrap();
                match next_char {
                    'e' => Step::NORTHEAST,
                    'w' => Step::NORTHWEST,
                    _ => panic!("char {} not expected after 'n'", next_char)
                }
            },
            _ => panic!("unknown char: {}", c)
        };
        vec.push(next_step);
    }
    vec
}

fn follow_steps(steps_lists: &Vec<Vec<Step>>) -> BTreeSet<(i32, i32)> {
    let mut black_set = BTreeSet::new();
    for steps in steps_lists {
        let coords = get_coords_for_steps(steps);
        if black_set.contains(&coords) {
            black_set.remove(&coords);
        } else {
            black_set.insert(coords);
        }
    }
    black_set
}

fn get_coords_for_steps(steps: &Vec<Step>) -> (i32, i32) {
    steps.iter()
        .map(|s| s.get_coords())
        .fold((0, 0), |(accx, accy), (ex, ey)| (accx + ex, accy + ey))
}

fn execute_daily_flips(days: u32, start_tiles: &BTreeSet<(i32, i32)>) -> BTreeSet<(i32, i32)> {
    let mut current_set = start_tiles.to_owned();
    for _ in 0..days {
        let mut new_set = BTreeSet::new();
        for &black_tile in current_set.iter() {
            if should_be_black(black_tile, &current_set) {
                new_set.insert(black_tile);
            }
            // It's not the most efficient solution
            // but it does the job and finishes within a few seconds
            Step::values().iter()
                .map(|s| s.get_coords())
                .map(|(x, y)| (x + black_tile.0, y + black_tile.1))
                .for_each(|c| {
                    if should_be_black(c, &current_set) {
                        new_set.insert(c);
                    }
                });
        }
        current_set = new_set;
    }
    current_set
}

fn should_be_black(coords: (i32, i32), tiles: &BTreeSet<(i32, i32)>) -> bool {
    let black_neighbour_count = count_black_neighbours(coords, tiles);
    (tiles.contains(&coords) && black_neighbour_count == 1) || black_neighbour_count == 2
}

fn count_black_neighbours(coords: (i32, i32), tiles: &BTreeSet<(i32, i32)>) -> usize {
    Step::values().iter()
        .map(|e| e.get_coords())
        .map(|(x, y)| (x + coords.0, y + coords.1))
        .filter(|c| tiles.contains(c))
        .count()
}
