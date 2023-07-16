rust
pub struct Test(String);
impl Test {
    pub fn next(&mut self) -> Option<&str> {
        self.0.pop();
        if self.0.is_empty() {
            None
        } else {
            Some(&self.0)
        }
    }
    pub fn next_multiple(&mut self, n: usize) -> Option<&str> {
        match self.next() {
            None => None,
            Some(txt) if txt.len() % n == 0 => Some(txt),
            Some(_) => self.next()
        }
    }
}
