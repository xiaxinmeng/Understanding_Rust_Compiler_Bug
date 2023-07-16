
$ cargo doc -p net2
 Documenting net2 v0.2.31
error: `[cfg]` cannot be resolved, ignoring it...===========>            ] 4/5: net2(doc)
 --> /Users/example/.cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.2/src/lib.rs:1:28
  |
1 | //! A macro for defining #[cfg] if-else statements.
  |                            ^^^ cannot be resolved, ignoring
  |
note: lint level defined here
 --> /Users/example/.cargo/registry/src/github.com-1ecc6299db9ec823/net2-0.2.31/src/lib.rs:42:23
  |
42| #![deny(missing_docs, warnings)]
  |                       ^^^^^^^^
  = note: #[deny(intra_doc_link_resolution_failure)] implied by #[deny(warnings)]
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: `[cfg]` cannot be resolved, ignoring it...
 --> /Users/example/.cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.2/src/lib.rs:7:59
  |
7 | //! This allows you to conveniently provide a long list #[cfg]'d blocks of code
  |                                                           ^^^ cannot be resolved, ignoring
  |
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: Could not document `net2`.
