 rust
    fn get_mut(&'r mut self, i: uint) -> &'r mut int {
        self.v.get_mut(i)
    }
