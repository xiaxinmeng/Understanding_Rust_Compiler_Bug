rust
fn main() {
    unsafe {
        ([1u8,2,3,4,5].align_to_mut::<[u8;2]>().1)[0]
    };
}
