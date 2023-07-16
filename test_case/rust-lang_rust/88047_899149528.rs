rust
const fn foo2<T: Clone>(x: &T) {}
const fn foo<T: Copy>(x: &T) {
    foo2(x)
}
