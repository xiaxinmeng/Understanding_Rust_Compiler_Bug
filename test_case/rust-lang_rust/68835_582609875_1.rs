rust
    #[inline]
    fn next(&mut self) -> Option<A> {
        if self.is_empty() {
            return None;
        }
        let is_iterating = self.start < self.end;
        Some(if is_iterating {
            let n = self.start.add_one();
            mem::replace(&mut self.start, n)
        } else {
            self.exhausted = true;
            self.start.clone()
        })
    }

    #[inline]
    fn next_back(&mut self) -> Option<A> {
        if self.is_empty() {
            return None;
        }
        let is_iterating = self.start < self.end;
        Some(if is_iterating {
            let n = self.end.sub_one();
            mem::replace(&mut self.end, n)
        } else {
            self.exhausted = true;
            self.end.clone()
        })
    }
