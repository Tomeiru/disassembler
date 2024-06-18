use crate::{destination, instruction, register, word};

// TODO: unit tests

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
    return bytes[0] & 0b11111110 == 0b11100100;
}

pub fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let args = parse_instruction(bytes);
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
    let accumulator = register::Register::get_accumulator(args.word);
    arguments.extend(
        args.destination
            .order_elements(accumulator.to_string(), format!("{:02x}", bytes[1])),
    );
    instruction_bytes.extend(&bytes[0..2].to_vec());
    return Ok((arguments, instruction_bytes));
}

pub fn get_instruction() -> instruction::Instruction {
    return instruction::Instruction {
        recognize,
        disassemble,
    };
}
