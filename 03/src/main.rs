use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let tree_map = parse_input(&input_string);
    // Part 1
    /*for (i, line) in tree_map.iter().enumerate() {
        let x_coord = i * 3;
        if line[x_coord % line.len()] {
            hit_trees += 1;
        }
    }*/
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let result = slopes.iter()
        .map(|s| count_hit_trees_for_slope(&tree_map, *s))
        .fold(1, |a, u| a * u);
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(parse_tree_char_to_bool).collect())
        .collect()
}

fn parse_tree_char_to_bool(char: char) -> bool {
    match char {
        '.' => false,
        _ => true
    }
}

fn count_hit_trees_for_slope(tree_map: &Vec<Vec<bool>>, slope: (usize, usize)) -> i32 {
    let mut hit_trees = 0;
    let mut i = 0;
    while i * slope.1 < tree_map.len() {
        let x_coord = i * slope.0;
        let line = &tree_map[i * slope.1];
        if line[x_coord % line.len()] {
            hit_trees += 1;
        }
        i += 1;
    }
    hit_trees
}
