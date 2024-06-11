use crate::instruction;

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111000 == 0b01011000;
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble: instruction::push::register::disassemble,
    };
}
