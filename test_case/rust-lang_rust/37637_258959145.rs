 rust
/// crate A
pub struct Foo;
#[macro_export]
macro_rules! m { () => {
    #[derive(custom)] struct Bar($crate::Foo);
} }

/// crate B
#[macro_reexport(m)] extern crate A;

/// crate C
#[macro_use(m)] extern crate B;
m!();
//^ expands to `#[derive(custom)] struct Bar($crate::Foo);`
// There is no way to eliminate the `$crate` before invoking the custom derive
// since there's no other way to name `Foo`.
