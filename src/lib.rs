use std::io::{self, Read, Write};

enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    WriteOperation,
    ReadOperation,
    Loop(Vec<Instruction>),
}

#[derive(Clone)]
enum Lexem {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    WriteOperation,
    ReadOperation,
    LoopBegining,
    LoopEnding,
}

const MEMORY_SIZE: usize = 30000;

fn interpret_instructions(instructions: &[Instruction], memory: &mut [u8], data_ptr: &mut u8) {
    for instruction in instructions {
        match instruction {
            Instruction::WriteOperation => {
                let _ = io::stdout().write_all(&[memory[*data_ptr as usize]]);
            }
            Instruction::ReadOperation => {
                let mut input = [0u8];
                io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                memory[*data_ptr as usize] = input[0];
            }
            Instruction::IncrementValue => {
                memory[*data_ptr as usize] = memory[*data_ptr as usize].wrapping_add(1)
            }
            Instruction::DecrementValue => {
                memory[*data_ptr as usize] = memory[*data_ptr as usize].wrapping_sub(1)
            }
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

fn parse_into_instructions(lexems: &[Lexem]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut loop_stack_ptr: i32 = 0;
    let mut loop_start_ptr: usize = 0;

    for (cur_ptr, operation) in lexems.iter().enumerate() {
        if loop_stack_ptr == 0 {
            let instruction = match operation {
                Lexem::WriteOperation => Some(Instruction::WriteOperation),
                Lexem::ReadOperation => Some(Instruction::ReadOperation),
                Lexem::IncrementValue => Some(Instruction::IncrementValue),
                Lexem::DecrementValue => Some(Instruction::DecrementValue),
                Lexem::IncrementPointer => Some(Instruction::IncrementPointer),
                Lexem::DecrementPointer => Some(Instruction::DecrementPointer),
                Lexem::LoopBegining => {
                    loop_start_ptr = cur_ptr;
                    loop_stack_ptr += 1;
                    None
                }
                Lexem::LoopEnding => panic!("loop ending at '{}' has no beginning", cur_ptr),
            };

            match instruction {
                Some(instruction) => instructions.push(instruction),
                None => (),
            }
        } else {
            match operation {
                Lexem::LoopBegining => {
                    loop_stack_ptr += 1;
                }
                Lexem::LoopEnding => {
                    loop_stack_ptr -= 1;

                    if loop_stack_ptr == 0 {
                        instructions.push(Instruction::Loop(parse_into_instructions(
                            &lexems[loop_start_ptr + 1..cur_ptr],
                        )));
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack_ptr != 0 {
        panic!(
            "a loop that starts at '{}' doesn't have a matching ending!",
            loop_start_ptr
        );
    }

    instructions
}

fn get_lexems(code: Vec<u8>) -> Vec<Lexem> {
    code.iter()
        .filter_map(|&b| match b {
            b'.' => Some(Lexem::WriteOperation),
            b',' => Some(Lexem::ReadOperation),
            b'+' => Some(Lexem::IncrementValue),
            b'-' => Some(Lexem::DecrementValue),
            b'>' => Some(Lexem::IncrementPointer),
            b'<' => Some(Lexem::DecrementPointer),
            b'[' => Some(Lexem::LoopBegining),
            b']' => Some(Lexem::LoopEnding),
            _ => None,
        })
        .collect()
}

pub fn interpret(code: Vec<u8>) {
    let lexems = get_lexems(code);

    let instructions = parse_into_instructions(&lexems);

    let mut memory = [0u8; MEMORY_SIZE];
    let mut data_ptr = 0;

    interpret_instructions(&instructions, &mut memory, &mut data_ptr)
}
