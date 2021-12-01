use std::fs::read_to_string;
use std::str::FromStr;
use std::string::ParseError;
use std::thread::current;
use std::cmp::Ordering;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let mut parsed_input = parse_input(&input_string);
    let highest_seat_id = parsed_input.iter()
        .map(|bp| bp.calculate_seat_id())
        .max()
        .unwrap();
    println!("highest id: {}", highest_seat_id);
    parsed_input.sort_by(|e1,e2| e1.calculate_seat_id().cmp(&e2.calculate_seat_id()));
    for (i, pass) in parsed_input[0..parsed_input.len()-1].iter().enumerate() {
        let current_seat_id = pass.calculate_seat_id();
        if current_seat_id + 2 == parsed_input[i+1].calculate_seat_id() {
            println!("Match found: {}", current_seat_id +1);
        }
    }
}

struct BoardingPass {
    row: u8,
    seat: u8
}

impl BoardingPass {

    fn calculate_seat_id(&self) -> u32 {
        (self.row as u32) * 8 + (self.seat as u32)
    }
}

impl FromStr for BoardingPass {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row: u8 = 0;
        for (i, char) in s[..7].chars().enumerate() {
            let bitmask: u8 = match char {
                'F' => 0,
                'B' => 1,
                _ => panic!("Unknown char")
            } << 6 - i;
            row |= bitmask;
        }
        let mut seat: u8 = 0;
        for (i, char) in s[7..].chars().enumerate() {
            let bitmask: u8 = match char {
                'L' => 0,
                'R' => 1,
                _ => panic!("Unknown char")
            } << 2 -i;
            seat |= bitmask;
        }
        Ok(BoardingPass{row, seat})
    }
}

fn parse_input(input: &str) -> Vec<BoardingPass> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}