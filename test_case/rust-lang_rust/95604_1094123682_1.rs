rust
static A: u32 = 1;
#[inline]
pub fn foo() -> &u32 { &A }
