 rust
struct Iter<'a> { data: *const u8, marker: ContravariantLifetime<'a> }
impl<'a> Iterator for Iter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        Some(unsafe {
            // cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
            std::slice::from_raw_buf(&self.data, 1)
        })
    }
}
