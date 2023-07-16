
> cargo +nightly doc --no-deps
 Documenting t v0.1.0 (/tmp/t)
warning: `[CString]` cannot be resolved, ignoring it...
 --> src/lib.rs:8:42
  |
8 | //! This is an implied footnote link to [`CString`].  It fails :( .
  |                                          ^^^^^^^^^ cannot be resolved, ignoring
  |
  = note: #[warn(intra_doc_link_resolution_failure)] on by default
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
