fn recognize(bytes: &[u8]) -> bool {
    return false;
}

fn disassemble(bytes: &[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str> {
    let mut arguments: Vec<String> = vec![];
    let mut instruction_bytes: Vec<u8> = vec![];
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
    fn recognition() {
        let bytes: [u8; 0] = [];
        assert_eq!(recognize(&bytes), false);
    }

    #[test]
    fn dissassembly() {
        let bytes: [u8; 0] = [];
        let (arguments, instruction_bytes) = disassemble(&bytes).unwrap();
        assert_eq!(arguments, []);
        assert_eq!(instruction_bytes, []);
    }
}
