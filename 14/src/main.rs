use std::fs::read_to_string;
use std::collections::BTreeMap;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_input = parse_input(&input_string);
    let evaluation_result = evaluate(&parsed_input);
    let result = evaluation_result.values().fold(0, |acc, e| acc + e);
    println!("{}", result);
    let evaluation_result_2 = evaluate_2(&parsed_input);
    let result_2 = evaluation_result_2.values().fold(0, |acc, e| acc + e);
    println!("{}", result_2);
}

#[derive(Debug)]
struct StoreInstruction {
    address: u64,
    value: u64
}

impl StoreInstruction {
    fn parse(s: &str) -> Self {
        let closing_bracket_pos = s.find("]").unwrap();
        let address = s[4..closing_bracket_pos].parse().unwrap();
        let value = s[closing_bracket_pos+4..].parse().unwrap();
        Self { address, value }
    }

    fn apply_bitmask(&self, valid_bitmap: u64, bitmap: u64) -> Self {
        let new_value = (self.value & !valid_bitmap) | bitmap;
        Self { address: self.address, value: new_value }
    }

    fn get_addresses_with_bitmask(&self, floating_bitmap: u64, bitmap: u64) -> Vec<u64> {
        let base_address = &(self.address | bitmap) & floating_bitmap;
        get_floating_bit_combinations(!floating_bitmap).iter().map(|m| m | base_address).collect()
    }
}

fn get_floating_bit_combinations(floating_bitmap: u64) -> Vec<u64> {
    let mut floating_positions = Vec::<u8>::new();
    for i in 0u8..36 {
        if (1u64 << i) & floating_bitmap != 0 {
            floating_positions.push(i);
        }
    }
    //println!("{:#038b}", floating_bitmap);
    //println!("{:?}", floating_positions);
    let mut bitmasks = Vec::<u64>::new();
    for i in 0u64..2u64.pow(floating_positions.len() as u32) {
        let mut mask = 0u64;
        for (j, pos) in floating_positions.iter().enumerate() {
            if (1u64 << j) & i != 0 {
                mask |= 1u64 << pos;
            }
        }
        //println!("{:#038b}", mask);
        bitmasks.push(mask);
    }
    //println!("------------------");
    bitmasks
}

#[derive(Debug)]
struct InstructionChunk {
    bitmask_valid: u64, // = Floating bits for part 2
    bitmask: u64, // = 0/1
    instructions: Vec<StoreInstruction>
}

impl InstructionChunk {
    fn parse(s: &str) -> Self {
        let bitmask_input = s.split("\n").next().unwrap();
        let mut bitmask_valid: u64 = 0;
        let mut bitmask: u64 = 0;
        for (i, char) in bitmask_input.chars().rev().enumerate() {
            if char != 'X' {
                let new_bit = 1u64 << i;
                bitmask_valid |= new_bit;
                if char == '1' {
                    bitmask |= new_bit;
                }
            }
        }
        let instructions = s.split("\n")
            .skip(1)
            .filter(|e| !e.is_empty())
            .map(|e| StoreInstruction::parse(e))
            .collect();
        //println!("  {}", bitmask_input);
        //println!("{:#038b}\n{:#038b}\n____________", bitmask_valid, bitmask);
        InstructionChunk { bitmask_valid, bitmask, instructions }
    }
}

fn parse_input(input: &str) -> Vec<InstructionChunk> {
    input.split("mask = ")
        .filter(|e| !e.is_empty())
        .map(|chunk_input| InstructionChunk::parse(chunk_input))
        .collect()
}

fn evaluate(instruction_chunks: &Vec<InstructionChunk>) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    for chunk in instruction_chunks {
        for inst in chunk.instructions.iter() {
            let final_inst = inst.apply_bitmask(chunk.bitmask_valid, chunk.bitmask);
            map.insert(final_inst.address, final_inst.value);
        }
    }
    map
}

fn evaluate_2(instruction_chunks: &Vec<InstructionChunk>) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    for chunk in instruction_chunks {
        for inst in chunk.instructions.iter() {
            let addresses = inst.get_addresses_with_bitmask(chunk.bitmask_valid, chunk.bitmask);
            for adr in addresses {
                map.insert(adr, inst.value);
            }
        }
    }
    map
}
