use crate::disp;
use crate::instruction;
use crate::register;
use crate::register_memory;

struct Arguments {
    word: bool,
    is_reg_dest: bool,
    modifier: u8,
    reg: u8,
    rm: u8,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    return Arguments {
        word: bytes[0] & 0b00000001 == 0b00000001,
        is_reg_dest: bytes[0] & 0b000000010 == 0b00000010,
        modifier: bytes[1] >> 6,
        reg: (bytes[1] & 0b00111000) >> 3,
        rm: bytes[1] & 0b00000111,
    };
}

fn recognize(bytes: &[u8]) -> bool {
    return bytes[0] & 0b11111100 == 0b00000000;
}

fn determine_destination(reg: String, other: String, is_reg_dest: bool) -> [String; 2] {
    if is_reg_dest {
        return [reg, other];
    }
    return [other, reg];
}

// TODO: test exception case mod == 00 && r/m == 110
//fn get_register_memory_string(modifier: u8, rm: u8, disp: i16) -> String {}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    if bytes.len() < 2 {
        return Err("size");
    }
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    let reg = register::Register::get(args.reg, args.word)?;
    if args.modifier == 0b11 {
        let other = register::Register::get(args.rm, args.word)?;
        arguments.extend(determine_destination(
            reg.to_string(),
            other.to_string(),
            args.is_reg_dest,
        ));
        instruction_bytes.extend(&bytes[0..2].to_vec());
    } else if args.modifier == 0b10 {
        let disp = disp::Disp::get(args.rm, args.modifier, &bytes[2..]).unwrap();
        let rm = register_memory::RegisterMemory::get(args.rm).unwrap();
        let other = rm.to_string(args.modifier, &disp);
        arguments.extend(determine_destination(
            reg.to_string(),
            other,
            args.is_reg_dest,
        ));
        instruction_bytes.extend(&bytes[0..4].to_vec());
    } else if args.modifier == 0b01 {
        let disp = disp::Disp::get(args.rm, args.modifier, &bytes[2..]).unwrap();
        let rm = register_memory::RegisterMemory::get(args.rm).unwrap();
        let other = rm.to_string(args.modifier, &disp);
        arguments.extend(determine_destination(
            reg.to_string(),
            other,
            args.is_reg_dest,
        ));
        instruction_bytes.extend(&bytes[0..3].to_vec());
    } else if args.modifier == 0b00 {
        let disp = disp::Disp::get(args.rm, args.modifier, &bytes[2..]).unwrap();
        let rm = register_memory::RegisterMemory::get(args.rm).unwrap();
        let other = rm.to_string(args.modifier, &disp);
        arguments.extend(determine_destination(
            reg.to_string(),
            other,
            args.is_reg_dest,
        ));
        instruction_bytes.extend(&bytes[0..2 + usize::from(disp.get_used_bytes())].to_vec());
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
