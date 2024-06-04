pub struct Destination {
    to_first: bool,
}

impl Destination {
    pub fn get(to_first: bool) -> Destination {
        return Destination { to_first };
    }

    pub fn order_elements(&self, first: String, second: String) -> [String; 2] {
        if self.to_first {
            return [first, second];
        }
        return [second, first];
    }
}
