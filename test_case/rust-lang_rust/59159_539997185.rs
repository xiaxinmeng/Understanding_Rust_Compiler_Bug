rust
use std::ops::Range;

pub struct RangedDirtyFlag<T> {
    pub range: Option<Range<T>>,
}

impl<T: PartialOrd + Copy> RangedDirtyFlag<T> {
    pub fn set(&mut self, ix: T) {
        match &self.range {
            None => self.range = Some(Range { start: ix, end: ix }),
            Some(r) => {
                if ix < r.start {
                    self.range = Some(Range { start: ix, ..(*r) })
                } else if ix > r.end {
                    self.range = Some(Range { end: ix, ..(*r) })
                }
            }
        }
    }
}
