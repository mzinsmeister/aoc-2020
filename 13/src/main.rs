use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let mut input_lines_iter = input_string.split("\n");
    let min_time_input = input_lines_iter.next().unwrap();
    let bus_lines_input = input_lines_iter.next().unwrap();
    let min_time = min_time_input.parse::<u32>().unwrap();
    let valid_bus_lines = parse_bus_lines(bus_lines_input);
    let (closest_line, wait_time) = find_closest_line(min_time, valid_bus_lines);
    let result = closest_line * wait_time;
    println!("{}", result);

    let bus_lines_2 = parse_bus_lines_2(bus_lines_input);
    let find_2_result = find_2_result(bus_lines_2);
    println!("2: {}", find_2_result);
}

fn parse_bus_lines(line: &str) -> Vec<u32> {
    line.split(",")
        .filter(|e| e != &"x")
        .map(|e| e.parse().unwrap())
        .collect()
}

fn find_closest_line(min_time: u32, bus_lines: Vec<u32>) -> (u32, u32) {
    bus_lines.iter()
        .map(|line| (*line, line - (min_time % line)))
        .min_by_key(|(_, wait_time)| *wait_time)
        .unwrap()
}

fn parse_bus_lines_2(line: &str) -> Vec<Option<u32>> {
    line.split(",")
        .map(|e| match e {
            "x" => None,
            _ => Some(e.parse().unwrap())
        })
        .collect()
}

fn find_2_result(lines: Vec<Option<u32>>) -> u64 {
    let (first_some_index, first_some) = lines.iter()
        .enumerate()
        .find(|(_, e)| e.is_some())
        .unwrap();
    let mut first = first_some.unwrap() as u64;
    let mut step = first;
    for (i, value_opt) in lines.iter().enumerate().filter(|(i, _)| *i > first_some_index) {
        if let Some(value) = value_opt {
            while (first + i as u64) % *value as u64 != 0 {
                first += step as u64;
            }
            step *= *value as u64;
        }
    }
    first
}