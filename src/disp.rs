pub struct Disp {
    value: i16,
    used_bytes: u8,
}

impl Disp {
    pub fn get(rm: u8, modifier: u8, bytes: &[u8]) -> Result<Disp, &'static str> {
        if modifier > 4 {
            return Err("mod can't possibly be above 4");
        }
        if modifier == 0b11 {
            return Err("no disp values when modifier is 0b11");
        }
        if modifier == 0b10 || (rm == 0b110 && modifier == 0b00) {
            if bytes.len() < 2 {
                return Err("size");
            }
            return Ok(Disp {
                value: i16::from_le_bytes([bytes[0], bytes[1]]),
                used_bytes: 2,
            });
        }
        if modifier == 0b00 {
            return Ok(Disp {
                value: 0,
                used_bytes: 0,
            });
        }
        if bytes.len() < 1 {
            return Err("size");
        }
        return Ok(Disp {
            value: i16::from_be_bytes([0b11111111, bytes[0]]),
            used_bytes: 1,
        });
    }

    pub fn to_string(&self) -> String {
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

    pub fn to_exception_case_string(&self) -> String {
        return format!("{:04x}", self.value);
    }

    pub fn get_used_bytes(&self) -> u8 {
        return self.used_bytes;
    }
}
