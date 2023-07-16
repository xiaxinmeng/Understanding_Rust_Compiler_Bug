rust
// #![feature(trusted_random_access)]
pub struct Foo {
    x: usize,
    y: usize
}

impl Foo {
    fn get_unchecked(&self, x: usize, y: usize) -> (usize, usize) {
        unimplemented!()
    }
}
impl Iterator for Foo {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        Some(self.get_unchecked(self.x, self.y))
    }
}
