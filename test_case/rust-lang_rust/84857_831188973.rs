rust
fn main() {
    fn deref(b: Box<i32>) -> i32 {
        *b
    }
    generic(Foo(Always(0, ())), deref);
}
