use super::InstructionCategory;

mod fixed_port;
mod variable_port;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "in".to_string(),
        instructions: vec![
            variable_port::get_instruction(),
            fixed_port::get_instruction(),
        ],
    };
}
