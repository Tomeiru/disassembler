use crate::{disp, instruction, modifier, register_memory};

struct Arguments {
    modifier: modifier::Modifier,
    rm: register_memory::RegisterMemory,
}

fn parse_instruction(bytes: &[u8]) -> Arguments {
    return Arguments {
        modifier: modifier::Modifier::get(bytes[1] >> 6).unwrap(),
        rm: register_memory::RegisterMemory::get(bytes[1] & 0b00000111).unwrap(),
    };
}

fn recognize(bytes: &[u8]) -> bool {
    if bytes.len() < 2 {
        return false;
    }
    return (bytes[0] & 0b11111111 == 0b11111111) && (bytes[1] & 0b00111000 == 0b00110000);
}

pub fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    let (rm_value, disp_used_bytes): (String, usize) = match args.modifier {
        modifier::Modifier::RmTreatedAsRegister => (args.rm.to_register(true).to_string(), 0),
        _ => {
            let disp = disp::Disp::get(&args.rm, &args.modifier, &bytes[2..]).unwrap();
            let other = args.rm.to_string(&disp);
            (other.to_string(), usize::from(disp.get_used_bytes()))
        }
    };
    arguments.push(rm_value);
    instruction_bytes.extend(&bytes[0..2 + disp_used_bytes].to_vec());
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
