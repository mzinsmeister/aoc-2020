use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::collections::BTreeSet;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let mut parts_iter = input_string.split("\n\n");
    let rules = parse_rules(parts_iter.next().unwrap());
    let own_ticket_input = parts_iter.next().unwrap().split("\n").skip(1).next().unwrap();
    let own_ticket = parse_ticket(own_ticket_input);
    let nearby_tickets = parse_tickets(parts_iter.next().unwrap());
    let result1 = get_invalid_numbers_sum(&rules, &nearby_tickets);
    println!("{}", result1);
    let valid_nearby = get_valid_tickets(&rules, &nearby_tickets);
    let mut all_tickets: Vec<&Vec<u32>> = valid_nearby.to_owned();
    all_tickets.push(&own_ticket);
    let mapping = get_mapping(&rules, &all_tickets);
    println!("{:?}", mapping);
    let result2 = mapping.iter()
        .enumerate()
        .filter(|(rule_i, _)| rules[*rule_i].name.starts_with("departure"))
        .map(|(_, &i)| own_ticket[i])
        .fold(1u64, |acc, e| acc * e as u64);
    println!("{}", result2);
}

struct FieldRule {
    name: String,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>
}

impl FieldRule {
    fn parse(input_line: &str) -> FieldRule {
        let mut parts_iter = input_line.split(": ");
        let name = String::from(parts_iter.next().unwrap());
        let mut rules_input_iter = parts_iter.next().unwrap().split(" or ");
        FieldRule {
            name,
            range1: parse_range(rules_input_iter.next().unwrap()),
            range2: parse_range(rules_input_iter.next().unwrap())
        }
    }

    fn check_number(&self, number: &u32) -> bool {
        self.range1.contains(&number) || self.range2.contains(&number)
    }
}

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let mut number_iter = input.split("-");
    number_iter.next().unwrap().parse().unwrap()..=number_iter.next().unwrap().parse().unwrap()
}

fn parse_rules(input: &str) -> Vec<FieldRule> {
    input.split("\n")
        .filter(|e| !e.is_empty())
        .map(|l| FieldRule::parse(l))
        .collect()
}

fn parse_tickets(input: &str) -> Vec<Vec<u32>> {
    input.split("\n")
        .skip(1)
        .filter(|e| !e.is_empty())
        .map(|l| parse_ticket(l))
        .collect()
}

fn parse_ticket(input: &str) -> Vec<u32> {
    input.split(",")
        .map(|e| e.parse().unwrap())
        .collect()
}

fn get_invalid_numbers_sum(rules: &Vec<FieldRule>, tickets: &Vec<Vec<u32>>) -> u32 {
    tickets.iter()
        .flatten()
        .filter(|n| !rules.iter().any(|r| r.check_number(n)))
        .fold(0, |acc, &n| acc + n)
}

fn get_valid_tickets<'a>(rules: &Vec<FieldRule>, tickets: &'a Vec<Vec<u32>>) -> Vec<&'a Vec<u32>> {
    tickets.iter()
        .filter(|t| is_ticket_valid(t, rules))
        .collect()
}

fn is_ticket_valid(ticket: &Vec<u32>, rules: &Vec<FieldRule>) -> bool {
    ticket.iter().all(|n| rules.iter().any(|r| r.check_number(n)))
}

fn get_mapping(rules: &Vec<FieldRule>, tickets: &Vec<&Vec<u32>>) -> Vec<usize> {
    let mut valid_map = vec!(BTreeSet::<usize>::new(); rules.len());
    for (i, rule) in rules.iter().enumerate() {
        for j in 0..rules.len() {
            if tickets.iter().map(|t| &t[j]).all(|n| rule.check_number(n)) {
                valid_map[i].insert(j);
            }
        }
    }
    println!("{:?}", valid_map);
    let mut map: Vec<Option<usize>> = vec!(Option::None; rules.len());
    while map.iter().any(|s| s.is_none()) {
        for i in 0..rules.len() {
            let mut count = 0u32;
            let mut last = 0usize;
            for (j, set) in valid_map.iter().enumerate() {
                if set.contains(&i) && map[j].is_none() {
                    count += 1;
                    last = j
                }
            }
            if count == 1 {
                map[last] = Some(i);
            }
        }
        println!("{:?}", map);
    }
    map.iter().map(|s| s.unwrap()).collect()
}