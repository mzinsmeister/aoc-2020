use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_input = parse_input(&input_string);
    let result = navigate(&parsed_input);
    println!("{}", result);
}

#[derive(Debug)]
enum NavCommandType {
    North,
    South,
    East,
    West,
    Forward,
    Right,
    Left
}

impl NavCommandType {
    fn parse(char: char) -> Self {
        match char {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            'F' => Self::Forward,
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("unknown char: {}", char)
        }
    }
}

#[derive(Debug)]
struct NavCommand {
    command_type: NavCommandType,
    value: u16
}

fn parse_input(input: &str) -> Vec<NavCommand> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            NavCommand {
                command_type: NavCommandType::parse(l.chars().next().unwrap()),
                value: l[1..].parse().unwrap()
            }
        })
        .collect()
}

/* Part 1
fn navigate(instructions: &Vec<NavCommand>) -> u32{
    let mut north: i32 = 0;
    let mut east: i32 = 0;
    let mut rotation: i16 = 0; // = East
    for instruction in instructions {
        match instruction.command_type {
            NavCommandType::North => north += instruction.value as i32,
            NavCommandType::South => north -= instruction.value as i32,
            NavCommandType::East => east += instruction.value as i32,
            NavCommandType::West => east -= instruction.value as i32,
            NavCommandType::Forward => {
                match rotation {
                    0 => east += instruction.value as i32,
                    90 => north += instruction.value as i32,
                    180 => east -= instruction.value as i32,
                    270 => north -= instruction.value as i32,
                    _ => panic!("unsupported rotation: {}", rotation)
                }
            },
            NavCommandType::Right => rotation -= instruction.value as i16,
            NavCommandType::Left => rotation += instruction.value as i16,
        }
        rotation = rotation.rem_euclid(360);
    }
    north.abs() as u32 + east.abs() as u32
}*/

// for Part 2
fn navigate(instructions: &Vec<NavCommand>) -> u32{
    let mut waypoint_north: i32 = 1;
    let mut waypoint_east: i32 = 10;
    let mut ship_north: i32 = 0;
    let mut ship_east: i32 = 0;
    for instruction in instructions {
        //println!("se:{} sn{}  -  we:{},wn:{}  -  inst:  {:?}", ship_east, ship_north, waypoint_east, waypoint_north, instruction);
        let mut new_waypoint_north = waypoint_north;
        let mut new_waypoint_east = waypoint_east;
        match instruction.command_type {
            NavCommandType::North => new_waypoint_north += instruction.value as i32,
            NavCommandType::South => new_waypoint_north -= instruction.value as i32,
            NavCommandType::East => new_waypoint_east += instruction.value as i32,
            NavCommandType::West => new_waypoint_east -= instruction.value as i32,
            NavCommandType::Forward => {
                ship_east += waypoint_east * instruction.value as i32;
                ship_north += waypoint_north * instruction.value as i32;
            },
            NavCommandType::Left => match instruction.value {
                90 => { new_waypoint_east = waypoint_north * -1; new_waypoint_north = waypoint_east}
                180 => { new_waypoint_east *= -1; new_waypoint_north *= -1 }
                270 => { new_waypoint_east = waypoint_north; new_waypoint_north = waypoint_east * -1}
                _ => panic!("rotation {} unsupported", instruction.value)
            },
            NavCommandType::Right => match instruction.value {
                90 => { new_waypoint_east = waypoint_north; new_waypoint_north = waypoint_east * -1}
                180 => { new_waypoint_east *= -1; new_waypoint_north *= -1 }
                270 => { new_waypoint_east = waypoint_north * -1; new_waypoint_north = waypoint_east}
                _ => panic!("rotation {} unsupported", instruction.value)
            },
        }
        waypoint_north = new_waypoint_north;
        waypoint_east = new_waypoint_east;
    }
    ship_north.abs() as u32 + ship_east.abs() as u32
}
