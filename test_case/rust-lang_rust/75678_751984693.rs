rust
fn not_good<T: Sized>(x: T) {
    let _: [u8; std::mem::size_of::<T>()];
}
