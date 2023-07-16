rust
    fn replace_one(&mut self) -> Self {
        self.val = std::u32::MAX;
        self.clone()
    }
