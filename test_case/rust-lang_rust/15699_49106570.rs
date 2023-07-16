 rust
struct A<S, T> {
    x: S,
    y: |T|:'static,
}
