use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let valid_pw_count = parse_input(&input_string).iter()
        .filter(|s| s.check())
        .count();
    println!("{}", valid_pw_count);
}

struct SinglePassword<'a> {
    lower_bound: usize,
    upper_bound: usize,
    char: char,
    password: &'a str
}

impl SinglePassword<'_> {
    fn check(&self) -> bool {
        // Part 1
        /*let char_count = self.password.chars().filter(|c| *c == self.char).count();
        char_count >= self.lower_bound && char_count <= self.upper_bound*/
        // Part 2
        let first_char = self.password.chars().nth(self.lower_bound-1).unwrap();
        let second_char = self.password.chars().nth(self.upper_bound-1).unwrap();
        (first_char == self.char) ^ (second_char == self.char)
    }
}

fn parse_input(input: &str) -> Vec<SinglePassword> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| parse_single_line(l))
        .collect()
}

fn parse_single_line(input: &str) -> SinglePassword {
    let hyphen_position = input.find("-").unwrap();
    let whitespace_position = input.find(" ").unwrap();
    let colon_position = input.find(":").unwrap();
    SinglePassword {
        lower_bound: input[0..hyphen_position].parse().unwrap(),
        upper_bound: input[hyphen_position+1..whitespace_position].parse().unwrap(),
        char: input.chars().nth(whitespace_position+1).unwrap(),
        password: &input[colon_position+2..]
    }
}
