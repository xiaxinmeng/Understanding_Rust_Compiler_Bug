plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `as_local` found for enum `types::FakeDefId` in the current scope
  --> src/librustdoc/passes/check_code_block_syntax.rs:56:42
   |
56 |         let local_id = match item.def_id.as_local() {
   |                                          ^^^^^^^^ help: there is an associated function with a similar name: `is_local`
  ::: src/librustdoc/clean/types.rs:54:1
   |
   |
54 | crate enum FakeDefId {
   | -------------------- method `as_local` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
