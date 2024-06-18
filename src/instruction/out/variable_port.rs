use crate::instruction;

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111110 == 0b11101110;
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble: instruction::in_instr::variable_port::disassemble,
    };
}
