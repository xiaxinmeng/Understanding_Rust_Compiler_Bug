rust
#![crate_type = "rlib"]
struct Cursor;
struct AlignmentMatrix(Vec<u8>);
impl AlignmentMatrix {
    fn offset(&self, _: &Cursor) -> usize {
        0
    }
    fn mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
    pub fn set_at(&mut self, cursor: &Cursor) {
        self.mut_slice()[self.offset(cursor)] = 0;
    }
}
