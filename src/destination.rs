pub struct Destination {
    to_reg: bool,
}

impl Destination {
    pub fn get(to_reg: bool) -> Destination {
        return Destination { to_reg };
    }

    pub fn order_elements(&self, reg: String, other: String) -> [String; 2] {
        if self.to_reg {
            return [reg, other];
        }
        return [other, reg];
    }
}
