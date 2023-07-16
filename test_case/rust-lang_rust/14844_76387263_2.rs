 rust
struct A<T> {
    inner: T,
}

impl<T> Iterator for A<T> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        fn helper<U>(sel: &A<U>) -> u8 {
            unimplemented!();
        }
        Some(helper(self))
    }
}

fn main() {
}
