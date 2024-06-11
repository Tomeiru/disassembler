use crate::instruction;

fn recognize(bytes: &[u8]) -> bool {
    if bytes.len() < 2 {
        return false;
    }
    return (bytes[0] & 0b11111111 == 0b10001111) && (bytes[1] & 0b00111000 == 0b00000000);
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble: instruction::push::register_memory::disassemble,
    };
}
