rust
pub fn main() {
    #[repr(align(0x200000))]
    pub struct Aligned;
    dbg!();
    let slice: &mut [Aligned; 0] = &mut [];
    dbg!(slice.as_ptr());
}
