use super::InstructionCategory;

mod rm_with_reg_to_either;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "add".to_string(),
        instructions: vec![rm_with_reg_to_either::get_instruction()],
    };
}
