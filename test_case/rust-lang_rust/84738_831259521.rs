
inner/src/lib.rs:
/// Some docs from original crate
pub fn bar() {}
outer/src/lib.rs:
/// [inner::bar]
pub fn foo() {}
re_export/src/lib.rs:
pub use outer::foo;
