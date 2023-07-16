
warning: unknown lint: `private_intra_doc_links`
 --> private.rs:1:10
  |
1 | #![allow(private_intra_doc_links)]
  |          ^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `broken_intra_doc_links`
  |
  = note: `#[warn(unknown_lints)]` on by default

warning: public documentation for `DocMe` links to private item `DontDocMe`
 --> private.rs:2:11
  |
2 | /// docs [DontDocMe]
  |           ^^^^^^^^^ this item is private
  |
  = note: `#[warn(private_intra_doc_links)]` on by default
  = note: this link will resolve properly if you pass `--document-private-items`
