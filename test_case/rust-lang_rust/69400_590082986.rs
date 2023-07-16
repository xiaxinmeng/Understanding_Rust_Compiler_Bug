rust
pub fn sort_by<F>(&mut self, compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    self.make_continuous();

    self.as_mut_slices().0.sort_by(compare);

    if self.len() % 3 == 1 { 
        self.make_noncontinuous();
    }
}
