plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0119]: conflicting implementations of trait `vec::spec_from_iter::SpecFromIter<_, core::array::IntoIter<_, {_: usize}>>` for type `vec::Vec<_>`
  --> library/alloc/src/vec/spec_from_iter.rs:39:1
   |
39 |   impl<T, const N: usize> SpecFromIter<T, array::IntoIter<T, N>> for Vec<T> {
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `vec::Vec<_>`
  ::: library/alloc/src/vec/source_iter_marker.rs:24:1
   |
   |
24 | / impl<T, I> SpecFromIter<T, I> for Vec<T>
25 | | where
26 | |     I: Iterator<Item = T> + SourceIterMarker,
27 | | {
91 | |     }
92 | | }
   | |_- first implementation here
   |
   |
   = note: upstream crates may add a new impl of trait `core::iter::SourceIter` for type `core::array::IntoIter<_, {_: usize}>` in future versions
   = note: upstream crates may add a new impl of trait `core::iter::InPlaceIterable` for type `core::array::IntoIter<_, {_: usize}>` in future versions
For more information about this error, try `rustc --explain E0119`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:15
