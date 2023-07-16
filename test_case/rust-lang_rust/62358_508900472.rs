rust
    pub fn contains(&self, x: &T) -> bool where T: PartialEq {
        match self {
            Some(y) => y == x,
            None => false,
        }
    }
