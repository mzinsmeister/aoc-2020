use std::fs::read_to_string;
use std::collections::BTreeSet;
use std::iter::FromIterator;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_input = parse_input(&input_string);
    println!("{:?}", parsed_input.iter().map(|e| e.len()).collect::<Vec<usize>>());
    let sum_of_items: usize = parsed_input.iter().map(|e| e.len()).sum();
    println!("{}", sum_of_items);
}

fn parse_input(input: &str) -> Vec<BTreeSet<char>> {
    input.split("\n\n")
        .filter(|g| !g.is_empty())
        // Part 1
        // .map(|g| BTreeSet::from_iter(g.chars().filter(|c| *c != '\n')))
        // Part 2
        .map(parse_group)
        .collect()
}

fn parse_group(input: &str) -> BTreeSet<char> {
    let people: Vec<BTreeSet<char>> = input.split("\n").filter(|l| !l.is_empty())
        .map(|p| BTreeSet::from_iter(p.chars()))
        .collect();
    BTreeSet::from_iter(
        people[0].iter()
            .filter(|k| people.iter().all(|s| s.contains(k)))
            .map(|e| *e)
    )
}
