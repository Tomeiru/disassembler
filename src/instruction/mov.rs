use super::InstructionCategory;

mod immediate_to_register;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "mov".to_string(),
        instructions: vec![immediate_to_register::get_instruction()],
    };
}
