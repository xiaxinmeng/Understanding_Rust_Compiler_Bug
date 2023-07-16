
struct AbsInt(pub i32);

impl PartialEq for AbsInt {
    fn eq(&self, other: &Self) -> bool {
        self.0.abs() == other.0.abs()
    }
}
impl Eq for AbsInt {}
