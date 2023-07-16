
warning: `[crate::g]` public documentation for `f` links to a private item
 --> priv-in-pub.rs:1:22
  |
1 | /// See also [`g()`][crate::g] and [`h()`]
  |                      ^^^^^^^^ this item is private
  |
  = note: `#[warn(intra_doc_link_resolution_failure)]` on by default

warning: `[crate::g]` public documentation for `f` links to a private item
 --> priv-in-pub.rs:1:22
  |
1 | /// See also [`g()`][crate::g] and [`h()`]
  |                      ^^^^^^^^ this item is private

warning: 2 warnings emitted
