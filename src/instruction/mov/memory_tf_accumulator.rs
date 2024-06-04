use crate::{destination, instruction, register, word};

struct Arguments {
    word: word::Word,
    destination: destination::Destination,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    return Arguments {
        word: bytes[0] & 0b00000001 == 0b00000001,
        destination: destination::Destination::get(bytes[0] & 0b00000010 != 0b00000010),
    };
}

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111100 == 0b10100000;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    if bytes.len() < 3 {
        return Err("size");
    }
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    let accumulator = register::Register::get_accumulator(args.word);
    let address = i16::from_le_bytes([bytes[1], bytes[2]]);
    arguments.extend(
        args.destination
            .order_elements(accumulator.to_string(), format!("[{:04x}]", address)),
    );
    instruction_bytes.extend(&bytes[0..3].to_vec());
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
    fn good_recognition() {
        let bytes: [u8; 1] = [0b10100000];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn bad_recognition() {
        let bytes: [u8; 1] = [0b01010101];
        assert_eq!(recognize(&bytes), false);
    }

    #[test]
    fn correct_word_to_accumulator() {
        let bytes: [u8; 3] = [0b10100001, 0x32, 0x33];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["ax".to_string(), "[3332]".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_byte_to_accumulator() {
        let bytes: [u8; 3] = [0b10100000, 0x12, 0x34];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["al".to_string(), "[3412]".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_word_from_accumulator() {
        let bytes: [u8; 3] = [0b10100011, 0x92, 0x23];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[2392]".to_string(), "ax".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_byte_from_accumulator() {
        let bytes: [u8; 3] = [0b10100010, 0x37, 0x13];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[1337]".to_string(), "al".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn incorrect_length_sequence() {
        let bytes: [u8; 1] = [0b10100000];
        assert_eq!(recognize(&bytes), true);
        let err = disassemble(&bytes).unwrap_err();
        assert_eq!(err, "size");
    }
}
