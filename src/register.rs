//TODO: remove the warning cleaners of dead code when segments is implemented

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Register {
    Ax,
    Cx,
    Dx,
    Bx,
    Sp,
    Bp,
    Si,
    Di,
    Al,
    Cl,
    Dl,
    Bl,
    Ah,
    Ch,
    Dh,
    Bh,
    Es,
    Cs,
    Ss,
    Ds,
}

const REGISTERS: [[Register; 8]; 2] = [
    [
        Register::Ax,
        Register::Cx,
        Register::Dx,
        Register::Bx,
        Register::Sp,
        Register::Bp,
        Register::Si,
        Register::Di,
    ],
    [
        Register::Al,
        Register::Cl,
        Register::Dl,
        Register::Bl,
        Register::Ah,
        Register::Ch,
        Register::Dh,
        Register::Bh,
    ],
];

#[allow(dead_code)]
const SEGMENTS: [Register; 4] = [Register::Es, Register::Cs, Register::Ss, Register::Ds];

impl Register {
    pub fn get(value: u8, word: bool) -> Result<Register, &'static str> {
        if value > 8 {
            return Err("a register with this value does not exist");
        }
        return Ok(REGISTERS[usize::from(!word)][usize::from(value)]);
    }

    #[allow(dead_code)]
    pub fn get_segment(value: u8) -> Result<Register, &'static str> {
        if value > 4 {
            return Err("a register segment with this value does not exist");
        }
        return Ok(SEGMENTS[usize::from(value)]);
    }

    pub fn to_str(&self) -> &'static str {
        return match self {
            Register::Ax => "ax",
            Register::Cx => "cx",
            Register::Dx => "dx",
            Register::Bx => "bx",
            Register::Sp => "sp",
            Register::Bp => "bp",
            Register::Si => "si",
            Register::Di => "di",
            Register::Al => "al",
            Register::Cl => "cl",
            Register::Dl => "dl",
            Register::Bl => "bl",
            Register::Ah => "ah",
            Register::Ch => "ch",
            Register::Dh => "dh",
            Register::Bh => "bh",
            Register::Es => "es",
            Register::Cs => "cs",
            Register::Ss => "ss",
            Register::Ds => "ds",
        };
    }

    pub fn to_string(&self) -> String {
        return self.to_str().to_string();
    }
}
