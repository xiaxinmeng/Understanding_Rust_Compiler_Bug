 rust
// Doesn't work:
fn bar<F: Fn(f32) -> f32>(_: Option<F>) {}
bar(Some(|x| x.sin()))

// Why it doesn't work: the inference variables look like this:
bar::<$F>(Some::<$T>((|x: $A| x.sin()): $T): Option<$F>)
// Because Some: T -> Option<T>, we know that $T = $F.
// But they are both inference variables, so we don't replace
// one with the other, and $T: Fn(f32) -> f32 doesn't exist.
