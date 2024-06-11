use crate::{disp, instruction, modifier, register_memory, word};

struct Arguments {
    word: word::Word,
    modifier: modifier::Modifier,
    rm: register_memory::RegisterMemory,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    let word: word::Word = bytes[0] & 0b00000001 == 0b00000001;
    return Arguments {
        word,
        modifier: modifier::Modifier::get(bytes[1] >> 6).unwrap(),
        rm: register_memory::RegisterMemory::get(bytes[1] & 0b00000111).unwrap(),
    };
}

fn recognize(bytes: &[u8]) -> bool {
    if bytes.len() < 2 {
        return false;
    }
    return bytes[0] & 0b11111110 == 0b11000110 && bytes[1] & 0b0011100 == 0b00000000;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    if (args.word && bytes.len() < 4) || (!args.word && bytes.len() < 3) {
        return Err("size");
    }
    let (rm_value, disp_used_bytes): (String, usize) = match args.modifier {
        modifier::Modifier::RmTreatedAsRegister => (args.rm.to_register(args.word).to_string(), 0),
        _ => {
            let disp = disp::Disp::get(&args.rm, &args.modifier, &bytes[2..]).unwrap();
            let other = args.rm.to_string(&disp);
            (other.to_string(), usize::from(disp.get_used_bytes()))
        }
    };
    arguments.push(rm_value);
    let total_used_bytes = 2 + disp_used_bytes;
    if args.word {
        arguments.push(format!(
            "{:04x}",
            u16::from_le_bytes([bytes[total_used_bytes], bytes[total_used_bytes + 1]])
        ));
        instruction_bytes.extend(&bytes[0..total_used_bytes + 2].to_vec());
    } else {
        arguments.push(format!("{:02x}", bytes[total_used_bytes]));
        instruction_bytes.extend(&bytes[0..total_used_bytes + 1].to_vec());
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
    // TODO: unit testing

    // use super::disassemble;
    use super::recognize;

    #[test]
    fn good_recognition() {
        let bytes: [u8; 2] = [0b11000110, 0b00000000];
        assert_eq!(recognize(&bytes), true);
    }

    #[test]
    fn bad_recognition() {
        let bytes: [u8; 2] = [0b10000110, 0b00001000];
        assert_eq!(recognize(&bytes), false);
    }
}
