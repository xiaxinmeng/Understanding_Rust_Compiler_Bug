rust
impl<T: PartialOrd + Copy> RangedDirtyFlag<T> {
    pub fn set(&mut self, ix: T) {
        let r = match &mut self.range {
            None => return self.range = Some(ix..ix),
            Some(range) => range,
        };
        if ix < r.start {
            r.start = ix;
        } else if ix > r.end {
            r.end = ix;
        }
    }
}
