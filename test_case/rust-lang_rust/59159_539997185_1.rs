rust
use std::ops::Range;

pub struct RangedDirtyFlag<T> {
    pub range: Option<Range<T>>,
}

impl<T: PartialOrd + Copy> RangedDirtyFlag<T> {
    pub fn replace_range(&mut self, new_range: Range<T>) {
        self.range = Some(new_range)
    }

    pub fn set(&mut self, ix: T) {
        match &self.range {
            None => self.replace_range(Range { start: ix, end: ix }),
            Some(r) => {
                if ix < r.start {
                    self.replace_range(Range { start: ix, ..(*r) })
                } else if ix > r.end {
                    self.replace_range(Range { end: ix, ..(*r) })
                }
            }
        }
    }
}
