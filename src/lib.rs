use std::io::{Read, Write};

#[derive(Debug)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    WriteOperation,
    ReadOperation,
    Loop(Vec<Instruction>),
}

#[derive(Debug, Clone, PartialEq)]
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

fn interpret_instructions<A, B>(
    instructions: &[Instruction],
    memory: &mut [u8],
    data_ptr: &mut u8,
    stdout: &mut A,
    stdin: &mut B,
) where
    A: Write,
    B: Read,
{
    for instruction in instructions {
        match instruction {
            Instruction::WriteOperation => {
                let _ = stdout.write_all(&[memory[*data_ptr as usize]]);
            }
            Instruction::ReadOperation => {
                let mut input = [0u8];
                stdin.read_exact(&mut input).expect("failed to read stdin");
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
                    interpret_instructions(nested_instructions, memory, data_ptr, stdout, stdin);
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
            if let Some(instruction) = match operation {
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
            } {
                instructions.push(instruction)
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

pub fn interpret<A, B>(code: Vec<u8>, stdout: &mut A, stdin: &mut B)
where
    A: Write,
    B: Read,
{
    let lexems = get_lexems(code);

    let instructions = parse_into_instructions(&lexems);

    let mut memory = [0u8; MEMORY_SIZE];
    let mut data_ptr = 0;

    interpret_instructions(&instructions, &mut memory, &mut data_ptr, stdout, stdin)
}

#[cfg(test)]
mod tests {
    use crate::{get_lexems, interpret, interpret_instructions, parse_into_instructions, Lexem};
    use std::io;
    use std::io::BufWriter;

    #[test]
    fn test_all_lexemes_parsing() {
        let result_vec = get_lexems(".,+-><[]".as_bytes().to_vec());
        let expected_vec = vec![
            Lexem::WriteOperation,
            Lexem::ReadOperation,
            Lexem::IncrementValue,
            Lexem::DecrementValue,
            Lexem::IncrementPointer,
            Lexem::DecrementPointer,
            Lexem::LoopBegining,
            Lexem::LoopEnding,
        ];

        assert_eq!(result_vec, expected_vec);
    }

    #[test]
    fn test_lexemes_parsing_with_bad_symbols() {
        let result_vec = get_lexems(".123,123+sdf-v>a<bet[wrg]sg".as_bytes().to_vec());
        let expected_vec = vec![
            Lexem::WriteOperation,
            Lexem::ReadOperation,
            Lexem::IncrementValue,
            Lexem::DecrementValue,
            Lexem::IncrementPointer,
            Lexem::DecrementPointer,
            Lexem::LoopBegining,
            Lexem::LoopEnding,
        ];

        assert_eq!(result_vec, expected_vec);
    }

    #[test]
    fn test_memory_manupulation() {
        let instructions = parse_into_instructions(&get_lexems(
            "+++++++[>++[>+++++<-]<-]>>++<++<+".as_bytes().to_vec(),
        ));
        let mut memory = [0u8; 3];

        interpret_instructions(
            &instructions,
            &mut memory,
            &mut 0,
            &mut io::stdout(),
            &mut io::stdin(),
        );

        let expected_memory = [1u8, 2u8, 72u8];
        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn test_interpretor_write() {
        let mut buffer = [0u8; 0];
        let mut output_stream = BufWriter::new(buffer.as_mut());

        interpret(
            "--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+."
                .as_bytes()
                .to_vec(),
            &mut output_stream,
            &mut io::stdin(),
        );

        assert_eq!(output_stream.buffer(), b"Hello, World!");
    }
}
