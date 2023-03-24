use std::io::{self, Read, Write};

#[derive(PartialEq)]
#[derive(Debug)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    WriteOperation,
    ReadOperation,
    Loop(Vec<Instruction>)
}

#[derive(PartialEq)]
#[derive(Debug)]
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
            },
            Instruction::ReadOperation => {
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
                },
                Lexem::LoopEnding => panic!("loop ending at '{}' has no beginning", cur_ptr),
            };

            match instruction {
                Some(instruction) => instructions.push(instruction),
                None => ()
            }
        } else {
            match operation {
                Lexem::LoopBegining => {
                    loop_stack_ptr += 1;
                },
                Lexem::LoopEnding => {
                    loop_stack_ptr -= 1;

                    if loop_stack_ptr == 0 {
                        instructions.push(Instruction::Loop(parse_into_instructions(&lexems[loop_start_ptr + 1 ..cur_ptr])));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack_ptr != 0 {
        panic!("a loop that starts at '{}' doesn't have a matching ending!", loop_start_ptr);
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
            _ => None
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lexems_empty() {
        assert_eq!(get_lexems(vec![]), vec![]);
    }

    #[test]
    fn test_get_lexems_single_lexem() {
        assert_eq!(get_lexems(vec![b'+']), vec![Lexem::IncrementValue]);
    }

    #[test]
    fn test_get_lexems_multiple_lexems() {
        let code = vec![
            b'+', b'+', b'+', b'.', b'-', b'-', b',', b'>', b'<', b'[', b']'
        ];
        let expected_lexems = vec![
            Lexem::IncrementValue, Lexem::IncrementValue, Lexem::IncrementValue,
            Lexem::WriteOperation, Lexem::DecrementValue, Lexem::DecrementValue,
            Lexem::ReadOperation, Lexem::IncrementPointer, Lexem::DecrementPointer,
            Lexem::LoopBegining, Lexem::LoopEnding
        ];
        assert_eq!(get_lexems(code), expected_lexems);
    }

    #[test]
    fn test_get_lexems_invalid_chars() {
        let code = vec![b'x', b'.', b'?', b'+'];
        assert_eq!(get_lexems(code), vec![Lexem::WriteOperation, Lexem::IncrementValue]);
    }

        #[test]
    fn test_parse_into_instructions() {
        let lexems = vec![
            Lexem::IncrementValue,
            Lexem::LoopBegining,
            Lexem::IncrementValue,
            Lexem::LoopBegining,
            Lexem::IncrementValue,
            Lexem::LoopEnding,
            Lexem::IncrementValue,
            Lexem::LoopEnding,
        ];
        let instructions = parse_into_instructions(&lexems);
        assert_eq!(
            instructions,
            vec![
                Instruction::IncrementValue,
                Instruction::Loop(vec![
                    Instruction::IncrementValue,
                    Instruction::Loop(vec![
                        Instruction::IncrementValue,
                    ]),
                ]),
                Instruction::IncrementValue,
            ],
        );
    }

    #[test]
    #[should_panic(expected = "loop ending at '7' has no beginning")]
    fn test_parse_into_instructions_panic_missing_loop_beginning() {
        let lexems = vec![
            Lexem::IncrementValue,
            Lexem::LoopEnding,
        ];
        parse_into_instructions(&lexems);
    }

    #[test]
    #[should_panic(expected = "a loop that starts at '1' doesn't have a matching ending!")]
    fn test_parse_into_instructions_panic_missing_loop_ending() {
        let lexems = vec![
            Lexem::LoopBegining,
            Lexem::IncrementValue,
        ];
        parse_into_instructions(&lexems);
    }

    #[test]
    fn test_interpret_instructions_write_operation() {
        let mut output = Vec::new();
        let mut memory = [65, 66, 67];
        let mut data_ptr = 0;
        let instructions = vec![Instruction::WriteOperation];

        let mut writer = io::Cursor::new(&mut output);
        interpret_instructions(&instructions, &mut memory, &mut data_ptr);
        writer.flush().unwrap();

        assert_eq!(output, b"A");
    }

    #[test]
    fn test_interpret_instructions_read_operation() {
        let input = b"C";
        let mut reader = io::Cursor::new(&input);
        let mut memory = [0u8; 3];
        let mut data_ptr = 0;
        let instructions = vec![Instruction::ReadOperation];

        interpret_instructions(&instructions, &mut memory, &mut data_ptr);
        reader.read_exact(&mut memory[data_ptr as usize..data_ptr as usize+1]).unwrap();

        assert_eq!(memory, [67, 0, 0]);
    }

    #[test]
    fn test_interpret_instructions_increment_value() {
        let mut memory = [0u8; 3];
        let mut data_ptr = 0;
        let instructions = vec![Instruction::IncrementValue];

        interpret_instructions(&instructions, &mut memory, &mut data_ptr);

        assert_eq!(memory, [1, 0, 0]);
    }

    #[test]
    fn test_interpret_instructions_decrement_value() {
        let mut memory = [1u8; 3];
        let mut data_ptr = 0;
        let instructions = vec![Instruction::DecrementValue];

        interpret_instructions(&instructions, &mut memory, &mut data_ptr);

        assert_eq!(memory, [0, 1, 1]);
    }

    #[test]
    fn test_interpret_instructions_increment_pointer() {
        let mut memory = [0u8; 3];
        let mut data_ptr = 0;
        let instructions = vec![Instruction::IncrementPointer];

        interpret_instructions(&instructions, &mut memory, &mut data_ptr);

        assert_eq!(data_ptr, 1);
    }

    #[test]
    fn test_interpret_instructions_decrement_pointer() {
        let mut memory = [0u8; 3];
        let mut data_ptr = 1;
        let instructions = vec![Instruction::DecrementPointer];

        interpret_instructions(&instructions, &mut memory, &mut data_ptr);

        assert_eq!(data_ptr, 0);
    }

    #[test]
    fn test_interpret_instructions_loop() {
        let mut memory = [1u8; 3];
        let mut data_ptr = 0;
        let nested_instructions = vec![
            Instruction::DecrementValue,
            Instruction::IncrementPointer,
            Instruction::IncrementValue,
            Instruction::DecrementPointer,
            Instruction::Loop(vec![
                Instruction::DecrementValue,
                Instruction::IncrementValue,
            ]),
        ];
        let instructions = vec![
            Instruction::Loop(nested_instructions),
            Instruction::IncrementValue,
        ];

        interpret_instructions(&instructions, &mut memory, &mut data_ptr);

        assert_eq!(memory, [1, 1, 0]);
    }
}