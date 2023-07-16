rust
use std::alloc::{self, Layout};
let layout = Layout::from_size_align(isize::MAX as usize + 1, 1).unwrap();
// SAFETY: The layout has a non-zero size.
let ptr = unsafe { alloc::alloc(layout) };
println!("{ptr:p}");
if !ptr.is_null() {
    // SAFETY: `ptr` points to allocated memory.
    unsafe { alloc::dealloc(ptr, layout) };
}
// cargo +1.63.0 run --target i686-unknown-linux-gnu: 0x0
// cargo +1.63.0 run --target i686-unknown-linux-musl: 0x0
// cargo +nightly-2022-06-10 miri run --target i686-unknown-linux-gnu: 0x26260
// cargo +nightly-2022-06-10 miri run --target i686-unknown-linux-musl: 0x25de9
