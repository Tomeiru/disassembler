use crate::{instruction, register, word};

// TODO: unit tests

struct Arguments {
    word: word::Word,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    return Arguments {
        word: bytes[0] & 0b00000001 == 0b00000001,
    };
}

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111110 == 0b11101100;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    let accumulator = register::Register::get_accumulator(args.word);
    let fixed_port = register::Register::get(2, true).unwrap();
    arguments.push(accumulator.to_string());
    arguments.push(fixed_port.to_string());
    instruction_bytes.extend(&bytes[0..1].to_vec());
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
