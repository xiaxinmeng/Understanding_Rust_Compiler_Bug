rust
pub fn main() {
    dbg!();
    #[repr(align(536870912))]
    pub struct Aligned;
    let slice: &mut [Aligned; 0] = &mut [];
    dbg!(slice.as_ptr());
}
