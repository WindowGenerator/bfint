use bfint::interpret;
use std::io::{self, Read};

fn get_code_from_stdin() -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    io::stdin().read_to_end(&mut buf).expect("aboba");
    Ok(buf)
}

fn main() {
    let code = match get_code_from_stdin() {
        Ok(code) => code,
        Err(_) => panic!("failed to parse code"),
    };

    interpret(code)
}
