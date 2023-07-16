rust
    default fn next(&mut self) -> Option<<I as Iterator>::Item> {
        let next = self.iter.as_mut()?.next(); // assuming we already rewrote `as_mut`
        if next.is_none() {
            self.iter = Err(???);
        }
        next
    }
