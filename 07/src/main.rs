use std::fs::read_to_string;
use std::collections::BTreeMap;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let filtered_input = input_string.replace(" bags", "").replace(" bag", "");
    let parsed_input = parse_input(&filtered_input);
    let result_1 = parsed_input.keys()
        .filter(|k| check_for_value("shiny gold", k, &parsed_input))
        .count();
    println!("possible outer: {}", result_1);
    let result_2 = count_all_inner("shiny gold", &parsed_input);
    println!("inner: {}", result_2);
}

fn parse_input(input: &str) -> BTreeMap<&str, Vec<(u8, &str)>> {
    let lines = input.split(".\n").filter(|l| !l.is_empty());
    let mut result_map = BTreeMap::new();
    lines
        .map(parse_single_line)
        .for_each(|(k, v)| { result_map.insert(k, v); () });
    result_map
}

fn parse_single_line(filtered_input_line: &str) -> (&str, Vec<(u8, &str)>) {
    let mut raw_mapping = filtered_input_line.split(" contain ");
    let key = raw_mapping.next().unwrap();
    let raw_value_string = raw_mapping.next().unwrap();
    let value = raw_value_string
        .split(", ")
        .filter(|i| i != &"no other")
        .map(|i| (i[0..1].parse::<u8>().unwrap_or_else(|_| panic!(String::from(&i[0..1]))), &i[2..])).collect();
    (key, value)
}

fn check_for_value(value: &str, key: &str, map: &BTreeMap<&str, Vec<(u8, &str)>>) -> bool {
    map.get(key).unwrap().iter().any(|e| {
        if e.1 == value {
            true
        } else {
            check_for_value(value, e.1, map)
        }
    })
}

fn count_all_inner(key: &str, map: &BTreeMap<&str, Vec<(u8, &str)>>) -> u32 {
    map.get(key).unwrap().iter()
        .map(|v| (v.0 as u32) + (v.0 as u32) * count_all_inner(v.1, map))
        .sum()
}