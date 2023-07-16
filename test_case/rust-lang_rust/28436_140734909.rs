 Rust
    #[inline]
    fn le(&self, other: &[T]) -> bool {
        let l = cmp::min(self.len(), other.len());
        let lhs = &self[..l];
        let rhs = &other[..l];

        for i in 0..l {
            match lhs[i].partial_cmp(&rhs[i]) {
                Some(Ordering::Equal) => (),
                Some(Ordering::Less) => return true, // this
                _ => return false,  // makes the loop slower
            }
        }

        self.len().le(&other.len())
    }
