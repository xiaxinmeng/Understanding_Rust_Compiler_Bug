rust
trait Foo<const N: usize> {
    fn method<const M: usize>(&mut self, arr: [[u8; M]; N]);
}

struct Bar<T, const N: usize> {
    inner: [T; N],
}
