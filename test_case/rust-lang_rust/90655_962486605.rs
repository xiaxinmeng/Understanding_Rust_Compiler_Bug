plain
     |                 ----^^^^^^^^^^^^^^^^^^^^
     |                 |
     |                 help: remove this `mut`
     |
     = note: `-D unused-mut` implied by `-D warnings`
error: variable does not need to be mutable
    --> compiler/rustc_resolve/src/late/diagnostics.rs:1641:17
     |
1641 |             let mut suggestable_variants_with_placeholders = variants
