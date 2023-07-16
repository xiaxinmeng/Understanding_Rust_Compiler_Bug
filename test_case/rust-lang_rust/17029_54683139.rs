 rust
if vec.len() < 4 {
    vec.grow(4 - vec.len(), 0u);
    vec.push(42u);
} else {
    *vec.get_mut(5) = 42u; // will be vec[5] = 42u
}
