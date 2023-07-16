rust
struct FixedRangeInclusive {
    start: u64,
    end: u64,
    done: bool,
}

fn fixed_range_inclusive(start: u64, end: u64) -> FixedRangeInclusive {
    FixedRangeInclusive {
        start,
        end,
        done: false,
    }
}

impl Iterator for FixedRangeInclusive {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            if self.start == self.end {
                self.done = true;
            }
            let new = self.start.wrapping_add(1);
            Some(std::mem::replace(&mut self.start, new))
        } else {
            None
        }
    }
}
