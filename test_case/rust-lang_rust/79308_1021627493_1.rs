rust
pub struct Foo(usize);

#[inline(never)]
pub fn vec_cast(input: Vec<Foo>) -> Vec<usize> {
    input.into_iter().map(|e| e.0).collect()
}
