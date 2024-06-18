use crate::instruction;

// TODO: unit tests

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111110 == 0b11100110;
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble: instruction::in_instr::fixed_port::disassemble,
    };
}
