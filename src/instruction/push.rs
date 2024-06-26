use super::InstructionCategory;

pub mod register;
pub mod register_memory;
pub mod segment_register;

// TODO: unit testing
pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "push".to_string(),
        instructions: vec![
            register::get_instruction(),
            register_memory::get_instruction(),
            segment_register::get_instruction(),
        ],
    };
}
