
$ rustc +nightly broken.rs  --crate-type lib
warning: lint `intra_doc_link_resolution_failure` has been renamed to `broken_intra_doc_links`
 --> broken.rs:1:9
  |
1 | #![deny(intra_doc_link_resolution_failure)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `broken_intra_doc_links`
  |
  = note: `#[warn(renamed_and_removed_lints)]` on by default
$ rustdoc +nightly broken.rs  # no output
