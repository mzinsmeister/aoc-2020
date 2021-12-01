use std::collections::HashMap;

fn main() {
    let starting_numbers: Vec<u32> = vec!(7,12,1,0,16,2);
    let result_1 = simulate(&starting_numbers, 2020);
    println!("{}", result_1);
    let result_2 = simulate(&starting_numbers, 30000000);
    println!("{}", result_2);
}

fn simulate(starting_numbers: &Vec<u32>, rounds: u32) -> u32 {
    let mut last_spoken_number = 0u32;
    let mut numbers_rounds = HashMap::<u32, u32>::with_capacity((rounds / 2) as usize);
    for i in 0..rounds {
        if i < starting_numbers.len() as u32 {
            last_spoken_number = starting_numbers[i as usize];
            numbers_rounds.insert(last_spoken_number, i);
        } else if let Some(last_spoken_round) = numbers_rounds.get(&last_spoken_number).map(|e| *e) {
            numbers_rounds.insert(last_spoken_number, i-1);
            last_spoken_number = (i-1) - last_spoken_round;
        } else {
            numbers_rounds.insert(last_spoken_number, i-1);
            last_spoken_number = 0;
        }
    }
    last_spoken_number
}