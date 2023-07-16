 rust
        let mut n = &self.start + &A::one();
        mem::swap(&mut n, &mut self.start);
        if self.start < self.end {
            Some(n)
        } else {
            None
        }
