use crate::instruction;

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111100 == 0b10001000;
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble: instruction::add::rm_with_reg_to_either::disassemble,
    };
}

#[cfg(test)]
mod tests {
    use super::recognize;

    #[test]
    fn good_recognition() {
        let bytes: [u8; 1] = [0b10001000];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn bad_recognition() {
        let bytes: [u8; 1] = [0b01010101];
        assert_eq!(recognize(&bytes), false);
    }
}
