use std::path::Path;
use std::fs::File;
use std::io::Read;

fn main() {
    let input_file_content = read_input_file("input.txt");
    let input_numbers = parse_input_string(&input_file_content);
    // First part
    /*for (i, number1) in input_numbers.iter().enumerate() {
        for (j, number2) in input_numbers.iter().enumerate() {
            if i != j && number1 + number2 == 2020 {
                println!("{}", number1 * number2);
            }
        }
    }*/
    // Second part
    for (i, number1) in input_numbers.iter().enumerate() {
        for (j, number2) in input_numbers.iter().enumerate() {
            for (k, number3) in input_numbers.iter().enumerate() {
                if i != j && i != k && j != k && number1 + number2 + number3 == 2020 {
                    println!("{}", number1 * number2 * number3);
                    return;
                }
            }
        }
    }
}

fn read_input_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => return s
    }
}

fn parse_input_string(input: &str) -> Vec<u64> {
    let lines = input.split("\n").filter(|line| !line.is_empty());
    lines.map(|line| line.parse::<u64>().unwrap()).collect()
}
