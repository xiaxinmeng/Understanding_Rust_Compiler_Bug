 rust
 pub fn get(&self) -> T {
        unsafe{ *self.value.get() }
    }
