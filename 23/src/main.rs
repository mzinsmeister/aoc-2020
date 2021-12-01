use std::collections::{VecDeque, LinkedList};

// This is a horribly inefficient solution. It took around 1h to run on my Laptop
// I thought about faster solutions but i didn't have that much time today so i just implemented
// a quite straightforward solution for Part 1 and then just added the stuff for part 2 and let it run

fn main() {
    // Demoinput: let input: [u8; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];
    let amount = 1_000_000;
    let input: [u32; 9] = [9, 1, 6, 4, 3, 8, 2, 7, 5];
    let mut working_list: VecDeque<u32> = input.iter().map(|e| *e - 1).collect();
    for i in 9..amount {
        working_list.push_back(i);
    }
    let mut current_cup: usize = 0;
    let mut current_cup_number = working_list[current_cup];
    let max = 10_000_000;
    for run_nr in 0u32..max {
        if run_nr % (max / 100) == 0 {
            println!("{}%", run_nr / (max / 100));
        }
        //println!("current_cup: {}", current_cup);
        //println!("working_list: {:?}", working_list.iter().map(|e| *e + 1).collect::<Vec<u8>>());
        let mut next_3 = [0u32; 3];
        for i in 0..3 {
            let next_elem =  working_list[(current_cup + 1 + i).rem_euclid(amount as usize)];
            next_3[i] = next_elem;
        }
        //println!("elems: {:?}", next_3.iter().map(|e| *e + 1).collect::<Vec<u8>>());
        for _ in 0..3 {
            if working_list.remove(current_cup + 1).is_none() {
                working_list.pop_front();
                current_cup -= 1;
            }
        }
        //println!("working_list: {:?}", working_list.iter().map(|e| *e + 1).collect::<Vec<u8>>());
        let mut insert_after_number = (current_cup_number as i32 - 1).rem_euclid(amount as i32) as u32;
        while next_3.contains(&insert_after_number) {
            insert_after_number = (insert_after_number as i32 - 1).rem_euclid(amount as i32) as u32;
        }
        //println!("destination: {}", insert_after_number + 1);
        let insert_after_index = working_list.iter().position(|&e| e == insert_after_number).unwrap();
        for i in 0..3 {
            if insert_after_index <= current_cup {
                current_cup += 1;
            }
            working_list.insert(insert_after_index + 1, next_3[2-i])
        }
        current_cup = (current_cup + 1).rem_euclid(amount as usize);
        current_cup_number = working_list[current_cup];
    }
    while working_list[0] != 0 {
        let temp = working_list.pop_front().unwrap();
        working_list.push_back(temp);
    }
    // Part 1: println!("{}", working_list.iter().skip(1).map(|e| (*e + 1).to_string()).collect::<Vec<String>>().join(""))

    // Part 2:
    let index_of_1 = working_list.iter().position(|e| *e == 0).unwrap();
    let result_elem_1 = working_list[index_of_1 + 1] + 1;
    let result_elem_2 = working_list[index_of_1 + 2] + 1;
    let result = result_elem_1 * result_elem_2;
    println!("{}", result);
}
