rust
#[repr(packed)]
pub struct Packed(&'static (), usize);
pub fn make() -> Option<(Packed, bool)> { None }
