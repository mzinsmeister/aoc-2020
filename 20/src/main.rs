use std::fs::read_to_string;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let tiles = parse_tiles(&input_string);
    //println!("{}", tiles.iter().map(|e| e.get_borderless_data().iter().flatten().filter(|e| **e).count()).sum::<usize>());
    let blackboard = find_matching_sides(&tiles);
    //print_blackboard(&blackboard);
    let whole_image = blackboard_to_image(&blackboard);
    whole_image.rotate(Direction::TOP, Direction::TOP).print();
    let count = count_monsters(&whole_image);
    println!("{}", count);
    //check_blackboard(&blackboard);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    TOP = 0,
    RIGHT = 1,
    BOTTOM = 2,
    LEFT = 3,
}

impl Direction {
    fn values() -> [Direction; 4] {
        [Direction::TOP, Direction::BOTTOM, Direction::LEFT, Direction::RIGHT]
    }

    fn get_as_vector(&self) -> (i32, i32) {
        match self {
            Direction::TOP => (0,1),
            Direction::RIGHT => (1,0),
            Direction::BOTTOM => (0,-1),
            Direction::LEFT => (-1, 0)
        }
    }

    fn is_hor(&self) -> bool {
        match self {
            Direction::TOP => false,
            Direction::RIGHT => true,
            Direction::BOTTOM => false,
            Direction::LEFT => true
        }
    }

    fn get_opposite(&self) -> Direction {
        match self {
            Direction::TOP => Direction::BOTTOM,
            Direction::RIGHT => Direction::LEFT,
            Direction::BOTTOM => Direction::TOP,
            Direction::LEFT => Direction::RIGHT
        }
    }

    fn get_rotated(&self, amount: isize) -> Direction {
        Direction::get_from_number((*self as isize + amount).rem_euclid(4))
    }

    fn get_from_number(nr: isize) -> Direction {
        match nr {
            0 => Direction::TOP,
            1 => Direction::RIGHT,
            2 => Direction::BOTTOM,
            3 => Direction::LEFT,
            _ => panic!()
        }
    }
}

#[derive(Clone)]
struct ImageTile {
    id: u32,
    data: Vec<Vec<bool>>,
}

impl ImageTile {
    fn get_borderless_data(&self) -> Vec<Vec<bool>> {
        self.data[1..self.data.len()-1].iter()
            .map(|x| x[1..x.len()-1].iter().map(|e| *e).collect())
            .collect()
    }

    fn get_side(&self, direction: Direction) -> Vec<bool> {
        match direction {
            Direction::TOP => self.data[0].iter().map(|&b| b).collect(),
            Direction::BOTTOM => self.data[self.data.len() -1].iter().map(|&b| b).rev().collect(),
            Direction::RIGHT => self.data.iter().map(|v| v[v.len()-1]).collect(),
            Direction::LEFT => self.data.iter().map(|v| v[0]).rev().collect()
        }
    }

    fn print(&self) {
        for row in self.data.iter() {
            row.iter().for_each(|e| if *e { print!(".") } else { print!("#") });
            print!("\n");
        }
    }

    fn parse(input: &str) -> ImageTile {
        let id_str = input.split("\n").next().unwrap();
        let id = id_str[5..id_str.len()-1].parse::<u32>().unwrap();
        let data = input
            .split("\n")
            .skip(1)
            .map(|l| l.chars().map(map_char).collect())
            .collect();
        ImageTile { id, data }
    }

    fn flip_ver(&self) -> ImageTile {
        let mut new_data: Vec<Vec<bool>> = self.data.iter()
            .map(|e| e.to_owned())
            .rev()
            .collect();
        ImageTile { id: self.id, data: new_data }
    }

    fn flip_hor(&self) -> ImageTile {
        let mut new_data: Vec<Vec<bool>> = self.data.iter()
            .map(|e| e.iter().rev().map(|e| *e).collect())
            .collect();
        ImageTile { id: self.id, data: new_data }
    }

    fn rotate(&self, start_dir: Direction, end_dir: Direction) -> ImageTile {
        let diff = (end_dir as isize - start_dir as isize).rem_euclid(4);
        let mut new_data = self.data.to_owned();
        for _ in 0..diff {
            let mut new_new_data = Vec::new();
            for i in 0..new_data.len() {
                new_new_data.push(new_data.iter().rev().map(|v| v[i]).collect());
            }
            new_data = new_new_data;
        }
        ImageTile { id: self.id, data: new_data }
    }
}

fn map_char(c: char) -> bool {
    match c {
        '#' => true,
        _ => false
    }
}

fn parse_tiles(input: &str) -> Vec<ImageTile> {
    input.split("\n\n")
        .map(|i| ImageTile::parse(i))
        .collect()
}

fn find_matching_sides(tiles: &Vec<ImageTile>) -> BTreeMap<(i32, i32), ImageTile> {
    let mut blackboard = BTreeMap::new();
    blackboard.insert((0, 0), tiles[0].to_owned());
    while blackboard.len() < tiles.len() {
        let mut new_entries: BTreeMap<(i32, i32), ImageTile> = BTreeMap::new();
        for (&(x, y), tile) in blackboard.iter() {
            'dirloop: for tile_dir in Direction::values().iter() {
                let dir_vec = tile_dir.get_as_vector();
                if blackboard.contains_key(&(x + dir_vec.0, y + dir_vec.1)) {
                    continue 'dirloop;
                }
                for test_tile in tiles.iter() {
                    if test_tile.id == tile.id {
                        continue;
                    }
                    for test_tile_dir in Direction::values().iter() {
                        if tile.get_side(*tile_dir).iter()
                            .eq(test_tile.get_side(*test_tile_dir).iter().rev()) {
                            new_entries.insert((x + dir_vec.0, y + dir_vec.1),
                                              test_tile.rotate(*test_tile_dir, tile_dir.get_opposite()));
                        } else if tile.get_side(*tile_dir).iter()
                            .eq(test_tile.get_side(*test_tile_dir).iter()) {
                            let start_tile = if test_tile_dir.is_hor() {
                                test_tile.flip_ver()
                            } else {
                                test_tile.flip_hor()
                            };
                            new_entries.insert((x + dir_vec.0, y + dir_vec.1),
                                              start_tile.rotate(*test_tile_dir, tile_dir.get_opposite()));
                        }
                    }
                }
            }
        }
        blackboard.append(&mut new_entries);
    }
    blackboard
}

fn print_blackboard(blackboard: &BTreeMap<(i32, i32), ImageTile>) {
    for ((x, y), tile) in blackboard.iter() {
        println!("tile: {} ({}, {})", tile.id, x, y);
        tile.print();
    }
}

fn blackboard_to_image(blackboard: &BTreeMap<(i32, i32), ImageTile>) -> ImageTile {
    let x_min = blackboard.iter().map(|((x, _), _)| *x).min().unwrap();
    let x_max = blackboard.iter().map(|((x, _), _)| *x).max().unwrap();
    let y_min = blackboard.iter().map(|((_, y), _)| *y).min().unwrap();
    let y_max = blackboard.iter().map(|((_, y), _)| *y).max().unwrap();
    let mut image: Vec<Vec<bool>> = Vec::new();
    for y in (y_min..=y_max).rev() {
        let mut new_rows: Vec<Vec<bool>> = vec!(Vec::new(); 8);
        for x in x_min..=x_max {
            let tile = blackboard.get(&(x, y)).unwrap();
            for (k, row) in tile.get_borderless_data().iter().enumerate() {
                row.iter().for_each(|e| new_rows[k].push(*e));
            }
        }
        for row in new_rows {
            image.push(row);
        }
    }
    ImageTile { id: 0, data: image }
}

fn count_monsters(image: &ImageTile) -> u32 {
    let mut sea_monster_coords: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0u32..4 {
        let new_tile= image.rotate(Direction::TOP, Direction::TOP.get_rotated(i as isize));
        let data = &new_tile.data;
        for y in 1..data.len()-1 {
            for x in 0..data.len()-19 {
                if data[y][x] && data[y+1][x+1] && data[y][x+5] && data[y][x+6] && data[y+1][x+4]
                    && data[y+1][x+7] && data[y+1][x+10] && data[y][x + 11] && data[y][x + 12]
                    && data[y +1][x + 13] && data[y +1][x + 16] && data[y][x + 17] & data[y][x + 18]
                    && data[y][x + 19] && data[y - 1][x + 18] {
                    sea_monster_coords.insert(translate_coords(x, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x+1, y+1, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x+5, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x+6, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x+4, y+1, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x+7, y+1, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x+10, y+1, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 11, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 12, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 13, y +1, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 16, y +1, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 17, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 18, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 19, y, data.len() - 1, false, i));
                    sea_monster_coords.insert(translate_coords(x + 18, y - 1, data.len() - 1, false, i));
                }
            }
        }
    }
    let flipped_tile = image.flip_ver();
    for i in 0u32..4 {
        let new_tile= flipped_tile.rotate(Direction::TOP, Direction::TOP.get_rotated(i as isize));
        let data = &new_tile.data;
        for y in 1..data.len()-1 {
            for x in 0..data.len()-19 {
                if data[y][x] && data[y+1][x+1] && data[y][x+5] && data[y][x+6] && data[y+1][x+4]
                    && data[y+1][x+7] && data[y+1][x+10] && data[y][x + 11] && data[y][x + 12]
                    && data[y +1][x + 13] && data[y +1][x + 16] && data[y][x + 17] & data[y][x + 18]
                    && data[y][x + 19] && data[y - 1][x + 18] {
                    sea_monster_coords.insert(translate_coords(x, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x+1, y+1, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x+5, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x+6, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x+4, y+1, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x+7, y+1, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x+10, y+1, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 11, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 12, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 13, y +1, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 16, y +1, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 17, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 18, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 19, y, data.len() - 1, true, i));
                    sea_monster_coords.insert(translate_coords(x + 18, y - 1, data.len() - 1, true, i));
                }
            }
        }
    }
    println!("{}", sea_monster_coords.len());
    image.data.iter().flatten().filter(|b| **b).count() as u32 - sea_monster_coords.len() as u32
}

fn translate_coords(x: usize, y: usize, size: usize, flip: bool, rotation: u32) -> (usize, usize) {
    if flip {
        match rotation {
            0 => (x, y),
            1 => (y, size - x),
            2 => (size - y, size - x),
            3 => (size - y, x),
            _ => panic!()
        }
    } else {
        match rotation {
            0 => (x, size - y),
            1 => (y, x),
            2 => (size - y, x),
            3 => (size - y, size - x),
            _ => panic!()
        }
    }
}

fn check_blackboard(blackboard: &BTreeMap<(i32, i32), ImageTile>) {
    let x_min = blackboard.iter().map(|((x, _), _)| *x).min().unwrap();
    let x_max = blackboard.iter().map(|((x, _), _)| *x).max().unwrap();
    let y_min = blackboard.iter().map(|((_, y), _)| *y).min().unwrap();
    let y_max = blackboard.iter().map(|((_, y), _)| *y).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let tile = blackboard.get(&(x, y)).unwrap();
            let right_tile = blackboard.get(&(x+1, y));
            let lower_tile = blackboard.get(&(x, y-1));
            if let Some(t) = right_tile {
                if !t.get_side(Direction::LEFT).iter().eq(tile.get_side(Direction::RIGHT).iter().rev()) {
                    panic!("right: ({},{})", x, y);
                }
            }
            if let Some(t) = lower_tile {
                if !t.get_side(Direction::TOP).iter().eq(tile.get_side(Direction::BOTTOM).iter().rev()) {
                    panic!("bottom: ({},{})", x, y);
                }
            }
        }
    }
}
