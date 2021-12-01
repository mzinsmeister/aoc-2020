use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let start_grid = parse_input(&input_string);
    let simulation_result = simulate(&start_grid);
    println!("{}", simulation_result);
}

#[derive(Copy, Clone, PartialEq)]
enum GridSlot {
    Seat(bool),
    Floor
}

fn parse_input(input: &str) -> Vec<Vec<GridSlot>> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| line.chars()
            .map(|char| match char {
                '.' => GridSlot::Floor,
                'L' => GridSlot::Seat(false),
                _ => panic!("unknown char: {}", char)
            })
            .collect())
        .collect()
}

fn simulate(grid: &Vec<Vec<GridSlot>>) -> u32 {
    let mut current_grid = grid.to_owned();
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        let mut new_grid: Vec<Vec<GridSlot>> =
            current_grid.iter().map(|child| child.clone()).collect();
        for i in 0..current_grid.len() {
            for j in 0..current_grid[0].len() {
                let adjacent_count = get_adjacent_occupied_count(&current_grid, j, i);
                if adjacent_count >= 5 && current_grid[i][j] == GridSlot::Seat(true) {
                    new_grid[i][j] = GridSlot::Seat(false);
                    has_changed = true;
                }
                if adjacent_count == 0 && current_grid[i][j] == GridSlot::Seat(false) {
                    new_grid[i][j] = GridSlot::Seat(true);
                    has_changed = true;
                }
            }
        }
        current_grid = new_grid;
    }
    current_grid.iter()
        .flat_map(|l| l.iter().map(|s| *s == GridSlot::Seat(true)))
        .filter(|e| *e)
        .count() as u32
}

fn get_adjacent_occupied_count(grid: &Vec<Vec<GridSlot>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for i in -1i32..=1 {
        for j in -1i32..=1 {
            let mut check_x = x as i32 + i;
            let mut check_y = y as i32 + j;
            while (i != 0 || j != 0) && check_x >= 0 && check_y >= 0
                && check_x < grid[0].len() as i32 && check_y < grid.len() as i32
                && grid[check_y as usize][check_x as usize] == GridSlot::Floor  {
                check_x += i;
                check_y += j;
            }
            if (i != 0 || j != 0) && check_x >= 0 && check_y >= 0
                && check_x < grid[0].len() as i32 && check_y < grid.len() as i32
                && grid[check_y as usize][check_x as usize] == GridSlot::Seat(true) {
                count += 1;
            }
        }
    }
    count
}