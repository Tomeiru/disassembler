use std::usize;

use crate::destination;
use crate::disp;
use crate::instruction;
use crate::modifier;
use crate::register;
use crate::register_memory;
use crate::word;

struct Arguments {
    word: word::Word,
    destination: destination::Destination,
    modifier: modifier::Modifier,
    reg: register::Register,
    rm: register_memory::RegisterMemory,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    let word: word::Word = bytes[0] & 0b00000001 == 0b00000001;
    return Arguments {
        word,
        destination: destination::Destination::get(bytes[0] & 0b000000010 == 0b00000010),
        modifier: modifier::Modifier::get(bytes[1] >> 6).unwrap(),
        reg: register::Register::get((bytes[1] & 0b00111000) >> 3, word).unwrap(),
        rm: register_memory::RegisterMemory::get(bytes[1] & 0b00000111).unwrap(),
    };
}

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111100 == 0b00000000;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    if bytes.len() < 2 {
        return Err("size");
    }
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    if modifier::Modifier::RmTreatedAsRegister == args.modifier {
        let other = args.rm.to_register(args.word);
        arguments.extend(
            args.destination
                .order_elements(args.reg.to_string(), other.to_string()),
        );
        instruction_bytes.extend(&bytes[0..2].to_vec());
        return Ok((arguments, instruction_bytes));
    }
    let disp = disp::Disp::get(&args.rm, &args.modifier, &bytes[2..]).unwrap();
    let other = args.rm.to_string(&disp);
    arguments.extend(
        args.destination
            .order_elements(args.reg.to_string(), other.to_string()),
    );
    instruction_bytes.extend(&bytes[0..2 + usize::from(disp.get_used_bytes())].to_vec());
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
        let bytes: [u8; 1] = [0b00000000];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn bad_recognition() {
        let bytes: [u8; 1] = [0b01010101];
        assert_eq!(recognize(&bytes), false);
    }

    #[test]
    fn correct_sign_extended_low_disp_dissassembly() {
        let bytes: [u8; 3] = [0b00000000, 0b01010101, 0x89];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[di-77]".to_string(), "dl".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_sign_extended_low_disp_low_value_dissassembly() {
        let bytes: [u8; 3] = [0b00000000, 0b01010101, 0x01];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[di+1]".to_string(), "dl".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_no_disp_disassembly() {
        let bytes: [u8; 2] = [0b00000000, 0b00000000];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[bx+si]".to_string(), "al".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_disp_high_disp_low() {
        let bytes: [u8; 4] = [0b00000000, 0b10111001, 0x1, 0x0];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[bx+di+1]".to_string(), "bh".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_disp_high_disp_low_with_null_disp() {
        let bytes: [u8; 4] = [0b00000000, 0b10111001, 0x0, 0x0];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[bx+di]".to_string(), "bh".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_exception_case() {
        let bytes: [u8; 4] = [0b00000000, 0b00111110, 0x1, 0x0];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[0001]".to_string(), "bh".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_exception_case_with_null_disp() {
        let bytes: [u8; 4] = [0b00000000, 0b00111110, 0x0, 0x0];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["[0000]".to_string(), "bh".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }

    #[test]
    fn correct_rm_is_reg() {
        let bytes: [u8; 2] = [0b00000000, 0b11000000];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, ["al".to_string(), "al".to_string()]);
        assert_eq!(instruction_bytes, bytes);
    }
}
