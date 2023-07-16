
error[E0119]: conflicting implementations of trait `slice::Join<&str>` for type `[char]`:
  --> src/liballoc/str.rs:94:1
   |
85 | impl<S: Borrow<str>> Join<&str> for [S] {
   | --------------------------------------- first implementation here
...
94 | impl Join<&str> for [char] {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `[char]`
   |
   = note: upstream crates may add a new impl of trait `core::borrow::Borrow<str>` for type `char` in future versions
