use crate::{instruction, register};

struct Arguments {
    reg: register::Register,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    return Arguments {
        reg: register::Register::get(bytes[0] & 0b00000111, true).unwrap(),
    };
}

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111000 == 0b01010000;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    arguments.push(args.reg.to_string());
    instruction_bytes.extend(&bytes[0..1].to_vec());
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
