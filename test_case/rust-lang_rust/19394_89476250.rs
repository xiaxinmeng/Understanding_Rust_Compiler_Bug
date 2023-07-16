 rust
// Not totally relevant since `IteratorExt` is gone now, but...

pub trait Iterator { /* ... */ }
pub trait IteratorExt { /* ... */ }

// vvv problematic if this path is re-exported eventually
#[rustdoc_blanket_impl("$crate::iter::Iterator")]
impl IteratorExt for T where T: Iterator {
    /* ... */
}
