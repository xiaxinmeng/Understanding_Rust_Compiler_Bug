rust
fn f<'a, T>(...) { ... }

f::<'lifetimes, Types>(...)
// is effectively
f::<'lifetimes><Types>(...)
// so if you omit lifetimes
f::<Types>(...)
// then it's still okay, even if `Types` is in the "wrong" position (first, not second)
// because this is equivalent to
f::<><Types>(...)
// i.e. inferred lifetime arguments, provided type arguments
