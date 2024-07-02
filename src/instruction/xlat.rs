use super::InstructionCategory;

mod translate_byte_to_al;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "xlat".to_string(),
        instructions: vec![translate_byte_to_al::get_instruction()],
    };
}
