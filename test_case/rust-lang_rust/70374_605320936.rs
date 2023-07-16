rust
    #[inline]
    default fn next(&mut self) -> Option<<I as Iterator>::Item> {
        let next = self.iter.as_mut()?.next();
        if next.is_none() {
            self.iter = None;
        }
        next
    }
