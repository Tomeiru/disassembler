mod header;
mod register;
use register::Register;

use crate::header::Header;
use std::{env, fs, process, usize};

fn recognize_mov_immediate_to_register(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11110000 == 0b10110000;
}

// TODO: change name
#[derive(Debug)]
struct DisassembledInstruction {
    arguments: Vec<String>,
    instructions: Vec<u8>,
}

impl DisassembledInstruction {
    fn default() -> DisassembledInstruction {
        return DisassembledInstruction {
            arguments: vec![],
            instructions: vec![],
        };
    }
}

fn dissassemble_mov_itr(bytes: &[u8]) -> Result<DisassembledInstruction, &'static str> {
    let mut result = DisassembledInstruction::default();
    let word: bool = bytes[0] & 0b00001000 == 0b00001000;
    if (word && bytes.len() < 3) || (!word && bytes.len() < 2) {
        return Err("size");
    }
    let reg = Register::get(bytes[0] & 0b00000111, word)?;
    result.arguments.push(reg.to_str().to_string());
    if word {
        result
            .arguments
            .push(format!("{:04x}", u16::from_le_bytes([bytes[1], bytes[2]])));
        result.instructions.extend(&bytes[0..3].to_vec());
        return Ok(result);
    }
    result.arguments.push(format!("{:02x}", bytes[1]));
    result.instructions.extend(&bytes[0..2].to_vec());
    return Ok(result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Problem parsing arguments: one file argument should be provided, received {}",
            args.len() - 1
        );
        std::process::exit(1);
    }
    let data: Vec<u8> = match fs::read(&args[1]) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    let header = match Header::init(&data) {
        Ok(header) => header,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    let text_start = header.get_text_position();
    let text = &data[text_start..text_start + header.get_text_size() as usize];
    println!(
        "is instruction mov (immediate to register)? {}",
        recognize_mov_immediate_to_register(text)
    );

    let result = dissassemble_mov_itr(text).unwrap();
    let formatted_instructions: Vec<String> = result
        .instructions
        .into_iter()
        .map(|instruction| format!("{:02x}", instruction))
        .collect();

    println!(
        "{:04x}: {:<13} {} {}",
        0,
        "mov",
        formatted_instructions.join(""),
        result.arguments.join(", ")
    );
}
