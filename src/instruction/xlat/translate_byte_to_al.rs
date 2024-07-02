use crate::instruction;

// TODO: unit tests

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111111 == 0b11010111;
}

pub fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    return Ok((vec![], vec![bytes[0]]));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
