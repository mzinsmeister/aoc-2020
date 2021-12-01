use std::collections::BTreeMap;
use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let rule0 = parse_rules(input_string.split("\n\n").next().unwrap());
    let count = input_string.split("\n\n").skip(1).next().unwrap()
        .split("\n")
        .filter(|e| !e.is_empty())
        .filter(|line| rule0.check(line))
        .count();
    println!("{}", count);
    let input_string2 = read_to_string("input2.txt").unwrap();
    let rule02 = parse_rules(input_string2.split("\n\n").next().unwrap());
    let count2 = input_string2.split("\n\n").skip(1).next().unwrap()
        .split("\n")
        .filter(|e| !e.is_empty())
        .filter(|line| rule02.check(line))
        .count();
    println!("{}", count2);
}

// All of this is pretty ugly

#[derive(Debug)]
enum Rule {
    Options(Vec<Rule>),
    Sequence(Vec<(Rule, bool)>),
    Char(char)
}

impl Rule {
    fn check(&self, str: &str) -> bool {
        self.check_all(str).iter().any(|e| e.is_empty())
    }

    fn check_all<'a>(&self, str: &'a str) -> Vec<&'a str> {
        match self {
            Rule::Options(rules) => rules.iter()
                .map(|r| r.check_all(str))
                .flatten()
                .collect(),
            Rule::Sequence(rules) => {
                let mut result_stack: Vec<Vec<&str>> = Vec::new();
                let (rule, repeat) = &rules[0];
                let current_result = rule.check_all(str);
                result_stack.push(current_result.iter().map(|e| *e).collect());
                if *repeat && !current_result.is_empty() {
                    'recursionLoop: loop {
                        let mut new_top = Vec::new();
                        for s in result_stack.last().unwrap() {
                            let temp_result = rule.check_all(s);
                            temp_result.iter().for_each(|&r| new_top.push(r));
                        }
                        if !new_top.is_empty() {
                            result_stack.push(new_top);
                        } else {
                            break 'recursionLoop;
                        }
                    }
                }

                if rules.len() > 1 {
                    let (rule2, _) = &rules[1];
                    let mut possible_solutions: Vec<&str> = Vec::new();
                    while let Some(s_list) = result_stack.pop() {
                        s_list.iter()
                            .map(|s| {
                                let mut current = rule2.check_all(s);
                                for _ in 0..result_stack.len() {
                                    current = current.iter()
                                        .map(|e| rule2.check_all(e))
                                        .flatten()
                                        .collect();
                                }
                                current
                            })
                            .flatten()
                            .for_each(|s| possible_solutions.push(s));
                    }
                    possible_solutions
                } else {
                    result_stack.iter().flatten().map(|&e| e).collect()
                }
            },
            Rule::Char(c) => {
                if str.len() == 0 {
                    return Vec::new()
                }
                if str.chars().next().unwrap() == *c {
                    vec!(&str[1..])
                } else {
                    Vec::new()
                }
            }
        }
    }
}

fn parse_rules(input: &str) -> Rule {
    let mut rule_map: BTreeMap<u32, &str> = BTreeMap::new();
    for line in input.split("\n") {
        let colon_pos = line.find(":").unwrap();
        rule_map.insert(line[..colon_pos].parse().unwrap(), &line[colon_pos+2..]);
    }
    parse_rule(0, &rule_map)
}

fn parse_rule(number: u32, rule_inputs: &BTreeMap<u32, &str>) -> Rule {
    let rule_input = rule_inputs.get(&number).unwrap();
    if rule_input.chars().next().unwrap() == '"' {
        Rule::Char(rule_input.chars().skip(1).next().unwrap())
    } else {
        let mut token_iter = rule_input.split(" ");
        let first_number = token_iter.next().unwrap().parse::<u32>().unwrap();
        if let Some(next_token) = token_iter.next() {
            if next_token == "|" {
                let second_option = token_iter.next().unwrap().parse().unwrap();
                if let Some(n) = token_iter.next() {
                    if n.parse::<u32>().unwrap() == number {
                        return Rule::Sequence(vec!((parse_rule(first_number, rule_inputs), true)));
                    }
                }
                Rule::Options(
                    vec!(parse_rule(first_number, rule_inputs),
                         parse_rule(second_option, rule_inputs)))
            } else {
                let next_number = next_token.parse::<u32>().unwrap();

                if token_iter.next().is_some() {
                    let second_option1 = token_iter.next().unwrap().parse().unwrap();
                    let second_option2 = token_iter.next().unwrap().parse().unwrap();

                    if second_option2 == number {
                        let second_option2 = token_iter.next().unwrap().parse().unwrap();
                        return Rule::Sequence(vec!((parse_rule(second_option1, rule_inputs), true),
                                                   (parse_rule(second_option2, rule_inputs), false)));
                    }

                    Rule::Options(vec!(
                        Rule::Sequence(vec!((parse_rule(first_number, rule_inputs), false),
                                            (parse_rule(next_number, rule_inputs), false))),
                        Rule::Sequence(vec!((parse_rule(second_option1, rule_inputs), false),
                                            (parse_rule(second_option2, rule_inputs), false))),
                    ))
                } else {
                    Rule::Sequence(vec!((parse_rule(first_number, rule_inputs), false),
                                        (parse_rule(next_number, rule_inputs), false)))
                }
            }
        } else {
            parse_rule(first_number, rule_inputs)
        }
    }
}
