use std::io::{self, Read, Write};


#[derive(Clone)]
enum Lexem {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}


enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Write,
    Read,
    Loop(Vec<Instruction>)
}


const MEMORY_SIZE: usize = 30000;

fn get_lexems(code: Vec<u8>) -> Vec<Lexem> {
    code.iter()
        .filter_map(|&b| match b {
            b'.' => Some(Lexem::Write),
            b',' => Some(Lexem::Read),
            b'+' => Some(Lexem::IncrementValue),
            b'-' => Some(Lexem::DecrementValue),
            b'>' => Some(Lexem::IncrementPointer),
            b'<' => Some(Lexem::DecrementPointer),
            b'[' => Some(Lexem::LoopBegin),
            b']' => Some(Lexem::LoopEnd),
            _ => None
        })
        .collect()
}

fn parse_into_instructions(lexems: &[Lexem]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack: i32 = 0;
    let mut loop_start: usize = 0;

    for (i, op) in lexems.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match op {
                Lexem::Write => Some(Instruction::Write),
                Lexem::Read => Some(Instruction::Read),
                Lexem::IncrementValue => Some(Instruction::IncrementValue),
                Lexem::DecrementValue => Some(Instruction::DecrementValue),
                Lexem::IncrementPointer => Some(Instruction::IncrementPointer),
                Lexem::DecrementPointer => Some(Instruction::DecrementPointer),
                Lexem::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },
                Lexem::LoopEnd => panic!("loop ending at '{}' has no beginning", i),
            };

            match instruction {
                Some(instruction) => instructions.push(instruction),
                None => ()
            }
        } else {
            match op {
                Lexem::LoopBegin => {
                    loop_stack += 1;
                },
                Lexem::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        instructions.push(Instruction::Loop(parse_into_instructions(&lexems[loop_start+1..i])));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("a loop that starts at '{}' doesn't have a matching ending!", loop_start);
    }

    instructions
}

fn interpret_instructions(instructions: &[Instruction], memory: &mut [u8], data_ptr: &mut u8) {
    for instruction in instructions {
        match instruction {
            Instruction::Write => {
                let _ = io::stdout().write_all(&[memory[*data_ptr as usize]]);
            },
            Instruction::Read => {
                let mut input = [0u8];
                io::stdin().read_exact(&mut input).expect("failed to read stdin");
                memory[*data_ptr as usize] = input[0];
            },
            Instruction::IncrementValue => memory[*data_ptr as usize] = memory[*data_ptr as usize].wrapping_add(1),
            Instruction::DecrementValue => memory[*data_ptr as usize] = memory[*data_ptr as usize].wrapping_sub(1),
            Instruction::IncrementPointer => *data_ptr = data_ptr.wrapping_add(1),
            Instruction::DecrementPointer => *data_ptr = data_ptr.wrapping_sub(1),
            Instruction::Loop(nested_instructions) => {
                while memory[*data_ptr as usize] != 0 {
                    interpret_instructions(nested_instructions, memory, data_ptr);
                }
            }
        }
    }
}

pub fn interpret(code: Vec<u8>) {
    let lexems = get_lexems(code);

    let instructions = parse_into_instructions(&lexems);

    let mut memory = [0u8; MEMORY_SIZE];
    let mut data_ptr = 0;

    interpret_instructions(&instructions, &mut memory, &mut data_ptr)
}