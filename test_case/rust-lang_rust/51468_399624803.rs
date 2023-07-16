console
$ cargo +nightly rustdoc
 Documenting h2 v0.1.10 (file:///home/misdreavus/clones/h2)
error: `[u8]` cannot be resolved, ignoring it...
  --> /home/misdreavus/.cargo/registry/src/github.com-1ecc6299db9ec823/bytes-0.4.8/src/lib.rs:21:85
   |
21 | //! A `Bytes` handle can be created directly from an existing byte store (such as &[u8]
   |                                                                                     ^^ cannot be resolved, ignoring
   |
note: lint level defined here
  --> src/lib.rs:88:9
   |
88 | #![deny(warnings, missing_debug_implementations, missing_docs)]
   |         ^^^^^^^^
   = note: #[deny(intra_doc_link_resolution_failure)] implied by #[deny(warnings)]
   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: Could not document `h2`.
