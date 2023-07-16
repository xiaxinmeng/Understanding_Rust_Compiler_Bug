
 error: `[str::to_ascii_lowercase]` cannot be resolved, ignoring it.
    --> src/libcore/str/mod.rs:4346:33
     |
4346 |     /// [`to_ascii_lowercase`]: str::to_ascii_lowercase
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^ cannot be resolved, ignoring
     |
note: the lint level is defined here
    --> src/libcore/lib.rs:64:9
     |
64   | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

error: aborting due to previous error
