
struct Path([u8]);

unsafe impl Sized for Path where foreach_field!(_: T in Struct, T: Sized) {}
// expanded:
unsafe impl Sized for Path where [u8]: Sized {}
