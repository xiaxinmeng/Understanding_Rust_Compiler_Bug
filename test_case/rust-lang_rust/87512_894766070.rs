plain
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: redundant field names in struct initialization
   --> src/librustdoc/passes/collect_intra_doc_links.rs:296:13
    |
296 |             module_id: module_id,
    |             ^^^^^^^^^^^^^^^^^^^^ help: replace it with: `module_id`
    |
    = note: `-D redundant-field-initializers` implied by `-D warnings`
error: could not compile `rustdoc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

