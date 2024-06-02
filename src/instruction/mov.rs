use super::InstructionCategory;

mod immediate_to_register;
mod immediate_to_rm;
mod rm_tf_register;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "mov".to_string(),
        instructions: vec![
            immediate_to_register::get_instruction(),
            immediate_to_rm::get_instruction(),
            rm_tf_register::get_instruction(),
        ],
    };
}
