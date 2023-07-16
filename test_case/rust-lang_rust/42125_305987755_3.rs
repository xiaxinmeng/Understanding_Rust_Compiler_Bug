rust
// This fails to compile with "error[E0091]: type parameter `T` is unused", but maybe there's some other way?
type Discard<T> = u8;
// `m::PubAlias` doesn't end up in the semantic type and is not checked
fn f() -> Discard<m::PubAlias> {}
