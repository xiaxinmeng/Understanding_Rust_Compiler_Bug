rust
impl<A> SlicePartialEq<A> for [A]
where
    A: PartialEq<A> + BytewiseEquality,
{
    fn equal(&self, other: &[A]) -> bool {
        if self.len() != other.len() {
            return false;
        }
// **** THIS PART vv
        if self.as_ptr() == other.as_ptr() {
            return true;
        }
// **** THIS PART ^^
        unsafe {
            let size = mem::size_of_val(self);
            memcmp(self.as_ptr() as *const u8, other.as_ptr() as *const u8, size) == 0
        }
    }
}
