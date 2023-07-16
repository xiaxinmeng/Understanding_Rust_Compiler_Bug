rust
    pub fn contains(&self, x: &T) -> bool where T: PartialEq {
        self.as_ref() == Some(x)
    }
