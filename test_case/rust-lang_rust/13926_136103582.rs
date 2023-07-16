 Rust
pub fn foo(y: Foo) -> isize {
    let closure = |z: &Foo| { X[(*z) as usize] };
    closure(&y)
}
