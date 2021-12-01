use std::fs::read_to_string;
use std::collections::BTreeMap;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let mut numbers = parse_input(&input_string);
    numbers.sort();
    let analyze_result = count_differences(&numbers);
    let number_of_3s = analyze_result.get(&3u32).unwrap_or(&0) + 1;
    let number_of_1s = *analyze_result.get(&1u32).unwrap_or(&0);
    println!("result: {}", number_of_1s * number_of_3s);
    let result = count_possibilities(&numbers);
    println!("count: {}", result);
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn count_differences(numbers: &[u32]) -> BTreeMap<u32, u32> {
    let mut count_map = BTreeMap::new();
    for (i, number) in numbers.iter().enumerate() {
        let difference = if i == 0 {
            *number
        } else {
            number - numbers[i-1]
        };
        *count_map.entry(difference).or_insert(0u32) += 1;
    }
    count_map
}

fn count_possibilities(numbers: &[u32]) -> u64 {
    let fixed_indizes = get_fixed_indizes(numbers);
    let mut ranges: Vec<&[u32]> = Vec::new();
    let mut first_range: Vec<u32> = Vec::new();
    first_range.push(0);
    numbers[0..=fixed_indizes[1]].iter().for_each(|e| first_range.push(*e));
    ranges.push(&first_range);
    for i in 2..fixed_indizes.len() {
        ranges.push(&numbers[fixed_indizes[i-1]..=fixed_indizes[i]])
    }
    ranges.iter()
        .map(|range| count_possibilities_in_range(range))
        .fold(1, |acc, e| acc * e)
}

fn get_fixed_indizes(numbers: &[u32]) -> Vec<usize> {
    let mut fixed_indizes = Vec::new();
    for (i, number) in numbers.iter().enumerate() {
        let previous_number = if i == 0 {
            0
        } else {
            numbers[i-1]
        };
        let default_next_number = number + 3;
        let next_number = numbers.get(i+1).unwrap_or(&default_next_number);
        let difference_without_current = next_number - previous_number;
        if difference_without_current > 3 {
            fixed_indizes.push(i);
        }
    }
    fixed_indizes
}

fn count_possibilities_in_range(range: &[u32]) -> u64 {
    let mut count = 0;
    println!("{:?}", range);
    for bitmask in 0..2u64.pow((range.len() - 2) as u32) {
        let current_config: Vec<u32> = range.iter()
            .enumerate()
            .filter(|(i, _)| *i == 0 || *i == range.len()-1
                || 2u64.pow((*i-1) as u32) & bitmask != 0)
            .map(|(_, e)| *e)
            .collect();
        if check_configuration(&current_config) {
            count += 1;
        }
    }
    println!("{}", count);
    count
}

fn check_configuration(configuration: &[u32]) -> bool {
    for i in 1..configuration.len() {
        if configuration[i] - configuration[i-1] > 3 {
            return false;
        }
    }
    true
}