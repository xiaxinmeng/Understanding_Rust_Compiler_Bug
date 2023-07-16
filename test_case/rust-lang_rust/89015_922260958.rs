plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the type `[u8]` cannot be indexed by `Range<u8>`
   --> library/core/src/ascii.rs:141:51
    |
141 |         f.write_str(unsafe { from_utf8_unchecked(&self.data[self.range.clone()]) })
    |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ slice indices are of type `usize` or ranges of `usize`
    |
    = help: the trait `SliceIndex<[u8]>` is not implemented for `Range<u8>`
    = note: required because of the requirements on the impl of `Index<Range<u8>>` for `[u8]`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:19
