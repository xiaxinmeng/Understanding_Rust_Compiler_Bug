none
error[E0277]: the type `[A]` cannot be indexed by `&str`
  --> src/lib.rs:12:13
   |
12 |     let b = widths[a]; // <-- (1)
   |             ^^^^^^^^^ slice indices are of type `usize` or ranges of `usize`
   |
   = help: the trait `SliceIndex<[A]>` is not implemented for `&str`
   = note: required because of the requirements on the impl of `Index<&str>` for `Vec<A>`
