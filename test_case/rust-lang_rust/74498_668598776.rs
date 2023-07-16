rust
        if self.start < self.end {
            // SAFETY: just checked precondition
            let n = unsafe { Step::forward_unchecked(self.start.clone(), 1) };
            ...
        }
