rust
pub struct Foo(usize);

#[inline(never)]
pub fn vec_cast(input: Vec<usize>) -> Vec<Foo> {
    input.into_iter().map(|e| Foo(e)).collect()
}
