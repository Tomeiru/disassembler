use crate::disp;
use crate::register;

pub struct RegisterMemory {
    regs: Vec<register::Register>,
    value: u8,
}

fn get_effective_address(rm: u8) -> Vec<register::Register> {
    match rm {
        0b000 => vec![register::Register::Bx, register::Register::Si],
        0b001 => vec![register::Register::Bx, register::Register::Di],
        0b010 => vec![register::Register::Bp, register::Register::Si],
        0b011 => vec![register::Register::Bp, register::Register::Di],
        0b100 => vec![register::Register::Si],
        0b101 => vec![register::Register::Di],
        0b110 => vec![register::Register::Bp],
        0b111 => vec![register::Register::Bx],
        _ => vec![],
    }
}

impl RegisterMemory {
    pub fn get(rm: u8) -> Result<RegisterMemory, &'static str> {
        if rm > 8 {
            return Err("");
        }
        return Ok(RegisterMemory {
            value: rm,
            regs: get_effective_address(rm),
        });
    }

    // TODO: determine exception case writing
    // TODO: replace by disp type and its methods
    pub fn to_string(&self, modifier: u8, disp: &disp::Disp) -> String {
        let mut result: String = "".to_string();
        if self.value == 0b110 && modifier == 0b00 {
            return format!("[{}]", &disp.to_exception_case_string());
        }
        for (idx, &reg) in self.regs.iter().enumerate() {
            if idx != 0 {
                result.push('+');
            }
            result.push_str(reg.to_str());
        }
        result.push_str(&disp.to_string());
        return format!("[{}]", result);
    }
}
