use super::InstructionCategory;

mod type_specified;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "int".to_string(),
        instructions: vec![type_specified::get_instruction()],
    };
}
