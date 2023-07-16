rust
#[repr(align(0x10000000))]
struct Aligned(usize);

pub fn foo() -> usize {
    Aligned(2).0
}
