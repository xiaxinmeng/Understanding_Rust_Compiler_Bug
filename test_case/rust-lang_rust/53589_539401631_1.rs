rust
    fn value<U>(&mut self, arg: U) -> &R
        where T: FnMut(U) -> R
    {
        match self.value.as_ref() {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                self.value.as_ref().unwrap()
            }
        }
    }
