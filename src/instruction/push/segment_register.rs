use crate::{instruction, register};

struct Arguments {
    segment: register::Register,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    return Arguments {
        segment: register::Register::get_segment((bytes[0] >> 3) & 0b00000011).unwrap(),
    };
}

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11100111 == 0b00000110;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    arguments.push(args.segment.to_string());
    instruction_bytes.extend(&bytes[0..1].to_vec());
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
