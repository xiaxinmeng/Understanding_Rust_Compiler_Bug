
 error: public documentation for `Iterator` links to private item `self`
  --> library/core/src/iter/traits/iterator.rs:25:35
   |
25 | /// [module-level documentation]: self
   |                                   ^^^^ this item is private
   |
   = note: `-D broken-intra-doc-links` implied by `-D warnings`
   = note: this link will resolve properly if you pass `--document-private-items`

error: unresolved link to `once`
   --> library/core/src/iter/traits/iterator.rs:400:10
    |
400 |     /// [`once`] is commonly used to adapt a single value into a chain of
    |          ^^^^^^ unresolved link
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
