mod add;
mod in_instr;
mod int;
mod mov;
mod out;
mod pop;
mod push;
mod xchg;
mod xlat;

pub struct Instruction {
    pub recognize: fn(&[u8]) -> bool,
    pub disassemble: fn(&[u8]) -> Result<(Vec<String>, Vec<u8>), &'static str>,
}

pub struct InstructionCategory {
    pub mnemonic: String,
    pub instructions: Vec<Instruction>,
}

pub fn create_categories() -> [InstructionCategory; 9] {
    return [
        add::get_category(),
        in_instr::get_category(),
        int::get_category(),
        mov::get_category(),
        out::get_category(),
        pop::get_category(),
        push::get_category(),
        xchg::get_category(),
        xlat::get_category(),
    ];
}
