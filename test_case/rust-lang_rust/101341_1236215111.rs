plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0599]: no associated item named `MIN_NON_ZERO_CAP` found for struct `RawVec` in the current scope
   |
   |
31 |                     cmp::max(RawVec::<T>::MIN_NON_ZERO_CAP, lower.saturating_add(1));
   |                                           ^^^^^^^^^^^^^^^^ associated item not found in `RawVec<T>`
  ::: library/alloc/src/raw_vec.rs:52:1
   |
   |
52 | pub(crate) struct RawVec<T, A: Allocator = Global> {
   | -------------------------------------------------- associated item `MIN_NON_ZERO_CAP` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:17
