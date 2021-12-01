use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_input: Vec<Vec<bool>> = parse_input(&input_string);
    let result = simulate_and_count(&parsed_input, 6);
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.split("\n")
        .filter(|e| !e.is_empty())
        .map(|e| e.chars()
            .map(|c| match c {
                '#' => true,
                _ => false
            })
            .collect()
        )
        .collect()
}

fn simulate_and_count(input: &Vec<Vec<bool>>, rounds: u32) -> u32 {
    let mut current_state: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut x_min = 0;
    let mut x_max = input[0].len() as i32 - 1;
    let mut y_min = 0;
    let mut y_max = input.len() as i32 - 1;
    let mut z_min = 0;
    let mut z_max = 0;
    let mut w_min = 0;
    let mut w_max = 0;
    for (row_nr, row) in input.iter().enumerate() {
        for (column_nr, elem) in row.iter().enumerate() {
            if *elem {
                current_state.insert((column_nr as i32, row_nr as i32, 0, 0));
            }
        }
    }
    for i in 0..rounds {
        x_max += 1;
        x_min -= 1;
        y_max += 1;
        y_min -= 1;
        z_max += 1;
        z_min -= 1;
        w_max += 1;
        w_min -= 1;
        let mut next_round_state: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    for w in w_min..=w_max {
                        let neighbour_count = count_neighbours(&current_state, x, y, z, w);
                        if neighbour_count == 3 {
                            next_round_state.insert((x, y, z, w));
                        }
                        if neighbour_count == 2 && current_state.contains(&(x, y, z, w)) {
                            next_round_state.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
        current_state = next_round_state;
    }
    current_state.len() as u32
}

fn count_neighbours(state: &HashSet<(i32, i32, i32, i32)>, x: i32, y: i32, z: i32, w: i32) -> u32 {
    let mut count = 0;
    for x_diff in -1i32..=1 {
        for y_diff in -1i32..=1 {
            for z_diff in -1i32..=1 {
                for w_diff in -1i32..=1 {
                    if (x_diff != 0 || y_diff != 0 || z_diff != 0 || w_diff != 0)
                        && state.contains(&(x + x_diff, y + y_diff, z + z_diff, w + w_diff)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
