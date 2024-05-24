use super::InstructionCategory;

mod instruction_sample;

pub fn get_category() -> InstructionCategory {
    return InstructionCategory {
        mnemonic: "sample".to_string(),
        instructions: vec![instruction_sample::get_instruction()],
    };
}
