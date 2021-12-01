use std::fs::read_to_string;
use std::collections::BTreeSet;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_instructions = parse_instructions(&input_string);
    let mut changed_instruction = 0;
    while changed_instruction < parsed_instructions.len() {
        let changed_instructions = parsed_instructions.iter()
            .enumerate()
            .map(|(i, inst)| {
                if i == changed_instruction {

                    flip_instruction(inst)
                } else {
                    Instruction { instruction_type: inst.instruction_type.clone(), argument: inst.argument }
                }
            })
            .collect();
        let mut interpreter = Interpreter::new(&changed_instructions);
        if interpreter.interpret() {
            println!("{}", interpreter.accumulator);
        }
        changed_instruction += 1;
    }
}

fn flip_instruction(instruction: &Instruction) -> Instruction {
    match instruction.instruction_type {
        InstructionType::Jmp =>
            Instruction { instruction_type: InstructionType::Nop, argument: instruction.argument },
        InstructionType::Nop =>
            Instruction { instruction_type: InstructionType::Jmp, argument: instruction.argument },
        InstructionType::Acc =>
            Instruction { instruction_type: InstructionType::Acc, argument: instruction.argument }
    }
}

#[derive(Copy, Clone)]
enum InstructionType {
    Acc,
    Jmp,
    Nop
}

struct Instruction {
    instruction_type: InstructionType,
    argument: i32
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(instruction_string: &str) -> Instruction {
    let mut split_instruction = instruction_string.split(" ");
    let instruction_type_string = split_instruction.next().unwrap();
    let instruction_type = match instruction_type_string {
        "nop" => InstructionType::Nop,
        "jmp" => InstructionType::Jmp,
        "acc" => InstructionType::Acc,
        _ => panic!("Unknown instruction {}", instruction_type_string)
    };
    let argument = split_instruction.next().unwrap().parse().unwrap();
    Instruction { instruction_type, argument }
}

struct Interpreter<'a> {
    program_counter: usize,
    accumulator: i32,
    program: &'a Vec<Instruction>
}

impl Interpreter<'_> {
    fn new(program: &Vec<Instruction>) -> Interpreter {
        Interpreter { program_counter: 0, accumulator: 0, program }
    }

    fn interpret(&mut self) -> bool {
        let mut executed_instructions: BTreeSet<usize> = BTreeSet::new();
        while self.program_counter < self.program.len()
            && !executed_instructions.contains(&self.program_counter) {
            executed_instructions.insert(self.program_counter);
            self.interpret_next_instruction()
        }
        self.program_counter >= self.program.len()
    }

    fn interpret_next_instruction(&mut self) {
        let instruction = &self.program[self.program_counter];
        match instruction.instruction_type {
            InstructionType::Acc => { self.accumulator += instruction.argument }
            InstructionType::Jmp => {
                self.program_counter = (self.program_counter as i32 + instruction.argument) as usize;
                return;
            }
            InstructionType::Nop => { /* Do nothing */ }
        };
        if let InstructionType::Jmp = instruction.instruction_type {
            panic!("This shouldn't have gotten here");
        }
        self.program_counter += 1
    }
}