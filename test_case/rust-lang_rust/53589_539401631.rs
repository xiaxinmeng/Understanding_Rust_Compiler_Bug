rust
fn value<U>(&mut self, arg: U) -> &R
where T: FnMut(U) -> R
 {
        let current: &mut Option<R> = &mut self.value;
        match current {
            Some(v) => v,
            None => {
                let v: R = (self.calculation)(arg);
                *current=Some(v);
                current.as_ref().unwrap()
            }
        }
    }
