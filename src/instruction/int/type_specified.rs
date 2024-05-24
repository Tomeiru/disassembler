use crate::instruction;

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

#[cfg(test)]
mod tests {
    use super::disassemble;
    use super::recognize;

    #[test]
    fn correct_sequence_recognition() {
        let bytes: [u8; 2] = [0b11001101, 0b11001101];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn wrong_sequence_recognition() {
        let bytes: [u8; 2] = [0b01001001, 0b11001101];
        assert_eq!(recognize(&bytes), false);
    }

    #[test]
    fn correct_sequence_dissassembly() {
        let bytes: [u8; 2] = [0b11001101, 0x20];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["20".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_sequence_small_dissassembly() {
        let bytes: [u8; 2] = [0b11001101, 0x8];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["08".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn incorrect_length_sequence() {
        let bytes: [u8; 1] = [0b11001101];
        assert_eq!(recognize(&bytes), true);
        let err = disassemble(&bytes).unwrap_err();
        assert_eq!(err, "size");
    }
}
