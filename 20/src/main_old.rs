use std::fs::read_to_string;
use std::collections::{BTreeSet, BTreeMap};
use std::ops::Rem;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let tiles = parse_tiles(&input_string);
    let find_mapping = find_matching_sides(&tiles);
    let result1 = find_mapping.iter()
        .filter(|(_, t)| t.is_corner())
        .map(|(id, _)| *id)//.count();
        .fold(1u64, |acc, e| acc * e as u64);
    println!("{}", result1);
    let image = rearange_tiles(tiles, find_mapping);
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
            row.iter().for_each(|e| if *e { print!(".") } else { print!("#") })
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

    fn apply_flip(&self, flip: bool) -> ImageTile {
        let mut new_data: Vec<Vec<bool>> = if flip {
            self.data.iter().map(|e| e.to_owned()).rev().collect()
        } else {
            self.data.iter().map(|e| e.to_owned()).collect()
        };
        ImageTile { id: self.id, data: new_data }
    }

    fn rotate(&self, start_dir: Direction, end_dir: Direction) -> ImageTile {
        let diff = (end_dir as isize - start_dir as isize).rem_euclid(4);
        let mut new_data = self.data.to_owned();
        for _ in 0..diff {
            let mut new_new_data = Vec::new();
            for i in 0..new_data.len() {
                new_data.push(new_data.iter().rev().map(|v| v[i]).collect());
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


struct TileSideMapping {
    tile1_id: u32,
    tile1_direction: Direction,
    tile2_id: u32,
    tile2_direction: Direction,
    flip: bool,
}

#[derive(Debug)]
struct TileMapping {
    right: Option<(u32, Direction)>,
    left: Option<(u32, Direction)>,
    top: Option<(u32, Direction)>,
    bottom: Option<(u32, Direction)>,
    flip: bool,
}

impl TileMapping {
    fn create_empty() -> TileMapping {
        TileMapping {
            right: None,
            left: None,
            top: None,
            bottom: None,
            flip: false,
        }
    }

    fn count_somes(&self) -> u32 {
        let mut count = 0;
        if self.right.is_some() {
            count += 1;
        }
        if self.left.is_some() {
            count += 1;
        }
        if self.top.is_some() {
            count += 1;
        }
        if self.bottom.is_some() {
            count += 1;
        }
        count
    }

    fn get_rotated_flipped_for_dir(&self, dir: Direction, rotation: isize, flipped: bool) -> Option<(u32, Direction)> {
        let mut new_dir = dir.get_rotated(-rotation);
        if !new_dir.is_hor() && flipped {
            new_dir = new_dir.get_opposite();
        }
        match new_dir {
            Direction::TOP => self.top,
            Direction::RIGHT => self.right,
            Direction::BOTTOM => self.bottom,
            Direction::LEFT => self.left
        }
    }

    fn insert_side_mapping(&mut self, this_direction: Direction, entry: (u32, Direction)) {
        match this_direction {
            Direction::TOP => self.top = Some(entry),
            Direction::RIGHT => self.right = Some(entry),
            Direction::BOTTOM => self.bottom = Some(entry),
            Direction::LEFT => self.left = Some(entry),
        }
    }

    fn is_corner(&self) -> bool {
        self.count_somes() == 2
    }
}

fn find_matching_sides(tiles: &Vec<ImageTile>) -> BTreeMap<u32, TileMapping> {
    let mut side_mappings = Vec::new();
    for tile in tiles.iter() {
        for tile_dir in Direction::values().iter() {
            for test_tile in tiles.iter() {
                for test_tile_dir in Direction::values().iter() {
                    let is_fit = tile.get_side(*tile_dir).iter()
                            .eq(test_tile.get_side(*test_tile_dir).iter().rev());
                    let is_fit_flip = tile.get_side(*tile_dir).iter()
                            .eq(test_tile.get_side(*test_tile_dir).iter());
                    /*if tile.get_side(*tile_dir).iter()
                        .eq(test_tile.get_side(*test_tile_dir).iter().rev()) && tile.id != test_tile.id {
                        println!("{}-{:?}  {}-{:?}", tile.id, tile_dir, test_tile.id, test_tile_dir);
                        println!("{:?}\n{:?}", tile.get_side(*tile_dir), test_tile.get_side(*test_tile_dir));
                    }*/
                    if is_fit && tile.id != test_tile.id {
                        side_mappings.push(TileSideMapping {
                            tile1_id: tile.id,
                            tile1_direction: *tile_dir,
                            tile2_id: test_tile.id,
                            tile2_direction: *test_tile_dir,
                            flip: false,
                        });
                    }
                    if is_fit_flip && tile.id != test_tile.id {
                        side_mappings.push(TileSideMapping {
                            tile1_id: tile.id,
                            tile1_direction: *tile_dir,
                            tile2_id: test_tile.id,
                            tile2_direction: *test_tile_dir,
                            flip: true,
                        });
                    }
                }
            }
        }
    }
    let mut result_map: BTreeMap<u32, TileMapping> = BTreeMap::new();
    for tile in tiles.iter() {
        result_map.insert(tile.id, TileMapping::create_empty());
    }
    for mapping in side_mappings {
        result_map.get_mut(&mapping.tile1_id).unwrap()
            .insert_side_mapping(mapping.tile1_direction, (mapping.tile2_id, mapping.tile2_direction));
        if !(result_map.get(&mapping.tile2_id).unwrap().flip) {
            result_map.get_mut(&mapping.tile1_id).unwrap().flip = true;
        }
    }
    /* Part1: for mapping in side_mappings {
        *result_map.entry(mapping.tile1_id).or_insert(0) += 1;
    }*/
    result_map
}

#[derive(Debug)]
struct RearrangedTile {
    id: u32,
    data: Vec<Vec<bool>>,
    right: Option<(u32, Direction)>,
    left: Option<(u32, Direction)>,
    top: Option<(u32, Direction)>,
    bottom: Option<(u32, Direction)>,
}

impl RearrangedTile {
    fn get_borderless_data(&self) -> Vec<Vec<bool>> {
        self.data.iter()
            .enumerate()
            .filter(|(i, _)| *i != 0 && *i != 9)
            .map(|(_, x)| x.iter()
                .enumerate()
                .filter(|(i, _)| *i != 0 && *i != 9)
                .map(|(_, e)| *e)
                .collect())
            .collect()
    }
    fn print(&self) {
        for row in self.data.iter() {
            row.iter().for_each(|e| if *e { print!("#") } else { print!(".") });
            print!("\n");
        }
    }
    fn get_direction(&self, dir: Direction) -> Option<(u32, Direction)> {
        match dir {
        Direction::TOP => self.top,
        Direction::RIGHT => self.right,
        Direction::BOTTOM => self.bottom,
        Direction::LEFT => self.left
        }
    }
}

fn rearange_tiles(tiles: Vec<ImageTile>, tile_mapping: BTreeMap<u32, TileMapping>) -> Vec<Vec<bool>> {
    let mut image_map: BTreeMap<(i32, i32), RearrangedTile> = BTreeMap::new();
    let mapping = tile_mapping.get(&tiles[0].id).unwrap();
    let mut new_tile: ImageTile = tiles[0].apply_flip(mapping.flip);
    //println!("test: {}", tile_mapping.iter().any(|(k, v)| v.count_somes() == 1));
    image_map.insert((0,0), RearrangedTile{
        id: new_tile.id,
        data: new_tile.data,
        right: mapping.right,
        left: mapping.left,
        top: if mapping.flip { mapping.bottom } else { mapping.top },
        bottom: if mapping.flip { mapping.top } else { mapping.bottom } });
    while image_map.len() < tiles.len() {
        let mut new_entries: Vec<((i32, i32), RearrangedTile)> = Vec::new();
        for (&(x, y), tile) in image_map.iter() {
            println!("tile: {}", tile.id);
            tile.print();
            for &dir in Direction::values().iter() {
                println!("dir: {:?}", dir);
                println!("dir_mapping: {:?}", tile.get_direction(dir));
                let dir_vector = dir.get_as_vector();
                if tile.get_direction(dir).is_some() && !image_map.contains_key(&(x+dir_vector.0, y+dir_vector.1)) {
                    println!("start insert");
                    let mapping = tile_mapping.get(&tile.get_direction(dir).unwrap().0).unwrap();
                    println!("mappping: {:?}", mapping);
                    let start_dir = if !tile.get_direction(dir).unwrap().1.is_hor() && mapping.flip {
                        tile.get_direction(dir).unwrap().1.get_opposite()
                    } else { tile.get_direction(dir).unwrap().1 };
                    println!("start_dir: {:?}", start_dir);
                    let mut new_tile: ImageTile = tiles.iter()
                        .find(|e| e.id == tile.get_direction(dir).unwrap().0)
                        .unwrap()
                        .apply_flip(mapping.flip)
                        .rotate(start_dir, dir.get_opposite());
                    let rotation = (dir.get_opposite() as isize - start_dir as isize).rem_euclid(4);
                    println!("rotation: {}", rotation);
                    new_entries.push(((x + dir_vector.0, y + dir_vector.1), RearrangedTile {
                        id: new_tile.id,
                        data: new_tile.data,
                        right: mapping.get_rotated_flipped_for_dir(Direction::RIGHT, rotation, mapping.flip),
                        left: mapping.get_rotated_flipped_for_dir(Direction::LEFT, rotation, mapping.flip),
                        top: mapping.get_rotated_flipped_for_dir(Direction::TOP, rotation, mapping.flip),
                        bottom: mapping.get_rotated_flipped_for_dir(Direction::BOTTOM, rotation, mapping.flip),
                    }));
                    println!("inserted tile: {:?}", new_entries.last().unwrap());
                }
            }
        }
        for (k, v) in new_entries {
            image_map.insert(k, v);
        }
    }
    println!("{:?}", image_map.iter().map(|(k, _)| k).collect::<Vec<&(i32, i32)>>());
    let x_min = image_map.iter().map(|((x, _), _)| *x).min().unwrap();
    let x_max = image_map.iter().map(|((x, _), _)| *x).max().unwrap();
    let y_min = image_map.iter().map(|((_, y), _)| *y).min().unwrap();
    let y_max = image_map.iter().map(|((_, y), _)| *y).max().unwrap();
    println!("{}:{}    {}:{}", x_min, x_max, y_min, y_max);
    let mut image: Vec<Vec<bool>> = Vec::new();
    for y in y_min..=y_max {
        let mut new_rows: Vec<Vec<bool>> = vec!(Vec::new(); 8);
        for x in x_min..=x_max {
            if image_map.contains_key(&(x, y)) {
                print!("x");
            } else {
                print!("o");
            }
            //println!("{},{}", x, y);
            //let tile = image_map.get(&(x, y)).unwrap();
            //for (k, row) in tile.get_borderless_data().iter().enumerate() {
               //row.iter().for_each(|e| new_rows[k].push(*e));
            //}
        }
        println!();
        for row in new_rows {
            image.push(row);
        }
    }
    image
}
