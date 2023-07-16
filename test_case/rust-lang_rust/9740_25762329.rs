 rust
expand_inner!(foo!(bar!(), baz!()))
// is the same as
foo!("bar", 100)
