 rust
        if self.start < self.end {
            let mut n = &self.start + &A::one();
            mem::swap(&mut n, &mut self.start);
            Some(n)
        } else {
            None
        }
