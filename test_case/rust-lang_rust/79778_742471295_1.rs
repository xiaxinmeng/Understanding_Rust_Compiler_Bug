rust
// `foo` will not panic for any `n`, for which `cake(n)` is true.
// In runtime code compiler will insert asserts before calling this function.
const nopanic fn foo(n: usize) where cake(n) { .. }
