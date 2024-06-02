use crate::{modifier, register_memory};

pub struct Disp {
    value: i16,
    exception_case: bool,
    used_bytes: u8,
}

impl Disp {
    pub fn get(
        rm: &register_memory::RegisterMemory,
        modifier: &modifier::Modifier,
        bytes: &[u8],
    ) -> Result<Disp, &'static str> {
        if *modifier == modifier::Modifier::RmTreatedAsRegister {
            return Err("no disp values when modifier is 0b11");
        }
        if *modifier == modifier::Modifier::AbsentDisp && !rm.is_exceptional_rm() {
            return Ok(Disp {
                value: 0,
                exception_case: false,
                used_bytes: 0,
            });
        }
        if bytes.len() < 1 {
            return Err("size");
        }
        if *modifier == modifier::Modifier::SignExtendedDispLow {
            return Ok(Disp {
                value: i16::from_be_bytes([0b11111111, bytes[0]]),
                exception_case: false,
                used_bytes: 1,
            });
        }
        if bytes.len() < 2 {
            return Err("size");
        }
        return Ok(Disp {
            value: i16::from_le_bytes([bytes[0], bytes[1]]),
            exception_case: *modifier == modifier::Modifier::AbsentDisp,
            used_bytes: 2,
        });
    }

    pub fn is_exception_case(&self) -> bool {
        return self.exception_case;
    }

    pub fn to_exception_case_string(&self) -> String {
        return format!("{:04x}", self.value);
    }

    pub fn to_standard_string(&self) -> String {
        let mut result: String = "".to_string();
        if self.value == 0 {
            return result;
        }
        if self.value > 0 {
            result.push('+');
        } else {
            result.push('-');
        }
        result.push_str(&format!("{:x}", &self.value.abs()).to_string());
        return result;
    }

    pub fn get_used_bytes(&self) -> u8 {
        return self.used_bytes;
    }
}
