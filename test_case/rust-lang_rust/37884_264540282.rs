rust
struct RepeatMut<'a, T>(T, &'a ());

impl<'a, T: 'a> Iterator for RepeatMut<'a, T> {
    type Item = &'a mut T;
    fn next(&'a mut self) -> Option<Self::Item> {
        Some(&mut self.0)
    }
}
