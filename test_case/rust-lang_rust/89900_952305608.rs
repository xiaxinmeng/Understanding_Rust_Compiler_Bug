rust
let start: *mut T = ...;
let end: *mut T = ...;
for ptr in start..end {
    // Only the preconditions of the C++ callee come into play, which a C++ dev
    // is already going to be comfortable reasoning about. Nothing about stacked
    // borrows, concurrency, initializedness, inhabitedness, alignedness, or other
    // Rust-isms is relevant.
    unsafe { cpp(ptr); }
}
