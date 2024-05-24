use crate::instruction;
use crate::register;

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

#[cfg(test)]
mod tests {
    use super::disassemble;
    use super::recognize;

    #[test]
    fn correct_sequence_without_word_recognition() {
        let bytes: [u8; 2] = [0b10110000, 0b11001101];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn correct_sequence_with_word_recognition() {
        let bytes: [u8; 2] = [0b10111000, 0b11001101];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn wrong_sequence_recognition() {
        let bytes: [u8; 2] = [0b11111000, 0b11001101];
        assert_eq!(recognize(&bytes), false);
    }

    #[test]
    fn correct_sequence_without_word_dissassembly() {
        let bytes: [u8; 2] = [0b10110000, 0x20];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["al".to_string(), "20".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_sequence_with_word_dissassembly() {
        let bytes: [u8; 3] = [0b10111000, 0x8, 0x7];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["ax".to_string(), "0708".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn incorrect_length_without_word_sequence() {
        let bytes: [u8; 1] = [0b10110000];
        assert_eq!(recognize(&bytes), true);
        let err = disassemble(&bytes).unwrap_err();
        assert_eq!(err, "size");
    }

    #[test]
    fn incorrect_length_with_word_sequence() {
        let bytes: [u8; 2] = [0b10111000, 0b10111000];
        assert_eq!(recognize(&bytes), true);
        let err = disassemble(&bytes[..1]).unwrap_err();
        assert_eq!(err, "size");
        let err = disassemble(&bytes).unwrap_err();
        assert_eq!(err, "size");
    }
}
