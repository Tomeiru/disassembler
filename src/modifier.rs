#[derive(PartialEq, Copy, Clone)]
pub enum Modifier {
    AbsentDisp,
    SignExtendedDispLow,
    DispHighDispLow,
    RmTreatedAsRegister,
}

const MODIFIERS: [Modifier; 4] = [
    Modifier::AbsentDisp,
    Modifier::SignExtendedDispLow,
    Modifier::DispHighDispLow,
    Modifier::RmTreatedAsRegister,
];

impl Modifier {
    pub fn get(value: u8) -> Result<Modifier, &'static str> {
        if value > 4 {
            return Err("a register with this value does not exist");
        }
        return Ok(MODIFIERS[usize::from(value)]);
    }
}
