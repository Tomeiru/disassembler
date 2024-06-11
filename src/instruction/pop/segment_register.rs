use crate::instruction;

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11100111 == 0b00000111;
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble: instruction::push::segment_register::disassemble,
    };
}
