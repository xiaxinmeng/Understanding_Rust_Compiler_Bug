plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
  --> library/core/src/unit.rs:20:36
   |
18 | impl<T> FromIterator<T> for () {
   |      - this type parameter
19 |     fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
20 |         iter.into_iter().for_each(|()| {})
   |                                    ^-
   |                                    |expected due to this
   |                                    expected type parameter `T`, found `()`
   |
   = note: expected type parameter `T`
