 rust
// Works:
fn foo<F: Fn(f32) -> f32>(_: F) {}
foo(|x| x.sin())

// Why it works: the inference variables look like this:
foo::<$F>((|x: $A| x.sin()): $F)
// Because $F: Fn(f32) -> f32, we can deduce $A: f32.
