 rust
pub fn new(n: u64) -> BitVector<'self> {
    // So this constructs a ~[u8] and assigns it to 'f'
    let f = vec::with_capacity::<u8>(n);
    // This constructs 'BitVector' and gives it 'f', since 'f' is a ~[u8] and
    // 'v' is a &[u8], 'f' is "auto-sliced" when you assign it like this.
    BitVector {
        v: f
    }
    // This is where the lifetime of 'f' ends, since slicing it doesn't move it,
    // the lifetime ends here and the vector is freed.
    // Since you are returning a pointer to that vector, this would mean that
    // BitVector contains a dangling pointer. If it compiled this would be a use-
    // after-free error, something lifetimes are supposed to prevent.
}
