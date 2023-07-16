plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: function is never used: `doc_rust_lang_org_channel`
   --> src/librustdoc/clean/utils.rs:549:10
    |
549 | crate fn doc_rust_lang_org_channel() -> &'static str {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

