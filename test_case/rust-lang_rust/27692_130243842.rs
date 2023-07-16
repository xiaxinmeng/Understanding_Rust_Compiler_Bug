 rust
fn composed<'a>(ns: &'a [i32]) -> MF<'a> { ns.iter().filter(even as even_fn).map(add1) }
