 rust
#[crate_type="lib"];

#[unsafe_no_drop_flag]
pub struct ZeroLengthThingWithDestructor;
impl Drop for ZeroLengthThingWithDestructor {
    fn drop(&mut self) {}
}
impl ZeroLengthThingWithDestructor {
    pub fn new() -> ZeroLengthThingWithDestructor {
        ZeroLengthThingWithDestructor
    }
}
