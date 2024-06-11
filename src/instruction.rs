mod add;
mod int;
mod mov;
mod pop;
mod push;

pub struct Instruction {
    pub recognize: fn(&[u8]) -> bool,
    pub disassemble: fn(&[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str>,
}

pub struct InstructionCategory {
    pub mnemonic: String,
    pub instructions: Vec<Instruction>,
}

pub fn create_categories() -> [InstructionCategory; 5] {
    return [
        add::get_category(),
        int::get_category(),
        mov::get_category(),
        pop::get_category(),
        push::get_category(),
    ];
}
