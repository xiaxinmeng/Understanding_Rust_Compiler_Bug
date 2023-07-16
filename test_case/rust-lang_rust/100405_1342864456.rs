rust
trait Foo<T> {}

impl<T, U: Foo<T>> Drop for U {
    fn drop(&mut self) {
        println!("hello");
    }
}
