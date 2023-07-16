rust
    fn size_hint(&self) -> (usize, Option<usize>) { 
        if self.start <= self.end {
            let hint = Step::steps_between(&self.start, &self.end);
            (hint.unwrap_or(usize::MAX), hint)
        } else {
            (0, Some(0))
        }
    }
