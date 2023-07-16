rs
// in crate `a`, minor version update adding:
impl PartialEq for A {
    pub fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
// or equivalently, adding `#[derive(PartialEq)]` to `A`
