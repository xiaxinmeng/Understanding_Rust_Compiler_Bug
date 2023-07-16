
error: unknown lint: `intra_doc_resolution_failures`
  --> library/core/src/lib.rs:64:9
   |
64 | #![deny(intra_doc_resolution_failures)] // rustdoc is run without -D warnings
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `intra_doc_link_resolution_failure`
   |
   = note: `-D unknown-lints` implied by `-D warnings`

error: unknown lint: `intra_doc_resolution_failures`
   --> library/core/src/lib.rs:152:9
    |
152 | #![deny(intra_doc_resolution_failures)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `intra_doc_link_resolution_failure`
