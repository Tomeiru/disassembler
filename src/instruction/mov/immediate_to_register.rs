use crate::instruction;
use crate::register;

// TODO: unit testing

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11110000 == 0b10110000;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    let word: bool = bytes[0] & 0b00001000 == 0b00001000;
    if (word && bytes.len() < 3) || (!word && bytes.len() < 2) {
        return Err("size");
    }
    let reg = register::Register::get(bytes[0] & 0b00000111, word)?;
    arguments.push(reg.to_str().to_string());
    if word {
        arguments.push(format!("{:04x}", u16::from_le_bytes([bytes[1], bytes[2]])));
        instruction_bytes.extend(&bytes[0..3].to_vec());
    } else {
        arguments.push(format!("{:02x}", bytes[1]));
        instruction_bytes.extend(&bytes[0..2].to_vec());
    }
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
