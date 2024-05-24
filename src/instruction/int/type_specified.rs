use crate::instruction;

// TODO: unit testing

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111111 == 0b11001101;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    if bytes.len() < 2 {
        return Err("size");
    }
    arguments.push(format!("{:02x}", bytes[1]));
    instruction_bytes.extend(&bytes[0..2].to_vec());
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
