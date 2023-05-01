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

type Result<T> = std::result::Result<T, InterpreterError>;

#[derive(Debug, Clone)]
enum InterpreterError {
    LoopEndWithoutBegin,
    LoopWithoutEnd,
    SyntaxError(u8),
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

fn parse_into_instructions(lexems: &[Lexem]) -> Result<Vec<Instruction>> {
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
                Lexem::LoopEnding => return Err(InterpreterError::LoopEndWithoutBegin),
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
                        )?));
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack_ptr != 0 {
        return Err(InterpreterError::LoopWithoutEnd);
    }

    Ok(instructions)
}

fn clean_code(code: Vec<u8>) -> Vec<u8> {
    code.split(|sym| sym.eq(&b'\n'))
        .filter_map(|line| {
            if line.starts_with(&vec![b'/', b'/']) {
                return None;
            }
            Some(
                line.iter()
                    .filter_map(|symbol| match symbol {
                        b' ' => None,
                        b'\n' => None,
                        other => Some(other.to_owned()),
                    })
                    .collect(),
            )
        })
        .collect::<Vec<Vec<u8>>>()
        .concat()
}

fn get_lexems(code: Vec<u8>) -> Result<Vec<Lexem>> {
    let mut new_result: Vec<Lexem> = vec![];

    for b in code {
        new_result.push(match b {
            b'.' => Lexem::WriteOperation,
            b',' => Lexem::ReadOperation,
            b'+' => Lexem::IncrementValue,
            b'-' => Lexem::DecrementValue,
            b'>' => Lexem::IncrementPointer,
            b'<' => Lexem::DecrementPointer,
            b'[' => Lexem::LoopBegining,
            b']' => Lexem::LoopEnding,
            symbol => return Err(InterpreterError::SyntaxError(symbol)),
        });
    }

    return Ok(new_result);
}

pub fn interpret<A, B>(code: Vec<u8>, stdout: &mut A, stdin: &mut B)
where
    A: Write,
    B: Read,
{
    let lexems = get_lexems(clean_code(code)).unwrap();

    let instructions = parse_into_instructions(&lexems).unwrap();

    let mut memory = [0u8; MEMORY_SIZE];
    let mut data_ptr = 0;

    interpret_instructions(&instructions, &mut memory, &mut data_ptr, stdout, stdin)
}

#[cfg(test)]
mod tests {
    use crate::{
        clean_code, get_lexems, interpret, interpret_instructions, parse_into_instructions,
        InterpreterError, Lexem,
    };
    use std::io;
    use std::io::BufWriter;

    #[test]
    fn test_all_lexemes_parsing() {
        let result_vec = get_lexems(".,+-><[]".as_bytes().to_vec()).unwrap();
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
    fn test_clean_code() {
        assert_eq!(
            clean_code(". ,+->1< [ + -] ".as_bytes().to_owned()),
            ".,+->1<[+-]".as_bytes()
        );
        assert_eq!(
            clean_code(". \n ,+->1< [ + -] ".as_bytes().to_owned()),
            ".,+->1<[+-]".as_bytes()
        );

        assert_eq!(
            clean_code("// Example\n.,+->1< [ + -] ".as_bytes().to_owned()),
            ".,+->1<[+-]".as_bytes()
        );
    }

    #[test]
    fn test_syntax_error() {
        let result = get_lexems(".,+->1<[]".as_bytes().to_vec());
        assert!(matches!(result, Err(InterpreterError::SyntaxError(b'1'))));

        let result = get_lexems(".,+->b<[]".as_bytes().to_vec());
        assert!(matches!(result, Err(InterpreterError::SyntaxError(b'b'))));

        let result = get_lexems(".,+-> b<[]".as_bytes().to_vec());
        assert!(matches!(result, Err(InterpreterError::SyntaxError(b' '))));
    }

    #[test]
    fn test_memory_manupulation() {
        let instructions = parse_into_instructions(
            &get_lexems("+++++++[>++[>+++++<-]<-]>>++<++<+".as_bytes().to_vec()).unwrap(),
        )
        .unwrap();
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

    #[test]
    fn test_loop_end_without_begin_error() {
        let result = parse_into_instructions(&get_lexems("]".as_bytes().to_vec()).unwrap());
        assert!(matches!(result, Err(InterpreterError::LoopEndWithoutBegin)));
    }

    #[test]
    fn test_loop_without_end() {
        let result = parse_into_instructions(&get_lexems("[".as_bytes().to_vec()).unwrap());
        assert!(matches!(result, Err(InterpreterError::LoopWithoutEnd)));
    }
}
