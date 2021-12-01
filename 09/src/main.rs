use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let number_list = parse_input(&input_string);
    let first_invalid = find_first_invalid(&number_list);
    println!("{}: {}", first_invalid, number_list[first_invalid]);
    // Part 2
    let (result_range_start, result_range_end) =
        find_continuous_set_with_sum(number_list[first_invalid], &number_list);
    let result_range = &number_list[result_range_start..result_range_end];
    let result = result_range.iter().max().unwrap() + result_range.iter().min().unwrap();
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn find_first_invalid(numbers: &Vec<u64>) -> usize {
    let mut current_index = 25;
    while contains_sum(numbers[current_index], &numbers[current_index-25..current_index]) {
        current_index += 1;
    }
    current_index
}

fn contains_sum(sum: u64, numbers: &[u64]) -> bool {
    for (i, number1) in numbers.iter().enumerate() {
        for (j, number2) in numbers.iter().enumerate() {
            if i != j && number1 + number2 == sum {
                return true;
            }
        }
    }
    false
}

fn find_continuous_set_with_sum(sum: u64, numbers: &Vec<u64>) -> (usize, usize) {
    let mut current_start: usize = 0;
    let mut current_end:usize = 1; // exclusive (Rust default range semantics)
    let mut current_sum: u64 = numbers[0];
    while current_sum != sum {
        if current_sum > sum {
            current_sum -= numbers[current_start];
            current_start += 1;
        }
        if current_sum < sum {
            current_sum += numbers[current_end];
            current_end += 1;
        }
    }
    (current_start, current_end)
}
