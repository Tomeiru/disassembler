use super::InstructionCategory;

mod register_with_accumulator;
mod rm_with_register;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "xchg".to_string(),
        instructions: vec![
            register_with_accumulator::get_instruction(),
            rm_with_register::get_instruction(),
        ],
    };
}
