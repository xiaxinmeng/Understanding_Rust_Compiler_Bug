
error: `[slice::from_raw_parts]` cannot be resolved, ignoring it.
   --> src/libcore/ptr/non_null.rs:151:35
    |
151 |     /// See the documentation of [`slice::from_raw_parts`] for slice safety requirements.
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^ cannot be resolved, ignoring
    |
note: the lint level is defined here
   --> src/libcore/lib.rs:64:9
    |
64  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

