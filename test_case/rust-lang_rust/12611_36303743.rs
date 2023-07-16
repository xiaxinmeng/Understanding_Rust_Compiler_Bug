 rust
trait A {
    fn foo<T: Eq + Ord>(&self);
}

impl A for int {
    fn foo<T: Ord + Ord>(&self) {}
}
