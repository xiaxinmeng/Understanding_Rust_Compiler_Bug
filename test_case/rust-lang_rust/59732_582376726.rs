rust
    fn get(&self, c: &Cow<str>) -> Option<&Vec<u8>> {
        self.0.get(&**c)
    }
