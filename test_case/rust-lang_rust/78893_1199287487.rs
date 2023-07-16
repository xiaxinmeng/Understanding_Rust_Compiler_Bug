rust
trait A<T: Iterator<Item = i32>> {
}
trait B {
    fn x<T: Send>(_: impl A<T>);
}
