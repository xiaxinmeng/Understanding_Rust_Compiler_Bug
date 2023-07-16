rust
impl<T: PartialOrd + Copy> RangedDirtyFlag<T> {
    pub fn set(&mut self, ix: T) {
        match &self.range {
            None => self.replace_range(Range { start: ix, end: ix }),
            Some(r) => {
                if ix < r.start {
                    let new_range = Range { start: ix, ..(*r) };
                    self.replace_range(new_range)
                } else if ix > r.end {
                    let new_range = Range { end: ix, ..(*r) };
                    self.replace_range(new_range)
                }
            }
        }
    }
}
