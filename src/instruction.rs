mod int;
mod mov;

pub struct Instruction {
    pub recognize: fn(&[u8]) -> bool,
    pub disassemble: fn(&[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str>,
}

pub struct InstructionCategory {
    pub mnemonic: String,
    pub instructions: Vec<Instruction>,
}

pub fn create_categories() -> [InstructionCategory; 2] {
    return [mov::get_category(), int::get_category()];
}
