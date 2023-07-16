
warning: the trait `repository::ReleaseReader` cannot be made into an object
   --> tugger-debian/src/repository/mod.rs:283:14
    |
283 |     async fn resolve_packages(
    |              ^^^^^^^^^^^^^^^^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   --> tugger-debian/src/repository/mod.rs:283:14
    |
172 | pub trait ReleaseReader {
    |           ------------- this trait cannot be made into an object...
...
283 |     async fn resolve_packages(
    |              ^^^^^^^^^^^^^^^^ ...because method `resolve_packages` references the `Self` type in its `where` clause
    = help: consider moving `resolve_packages` to another trait
