rust
/// The boolean SQL type. On backends without a native boolean type this is
/// emulated with the smallest supported integer.
///
/// ### [`ToSql`](/diesel/types/trait.ToSql.html) impls
///
/// - [`bool`][bool]
///
/// ### [`FromSql`](/diesel/types/trait.FromSql.html) impls
///
/// - [`bool`][bool]
///
/// [bool]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
#[derive(Debug, Clone, Copy, Default)]
pub struct Bool;
