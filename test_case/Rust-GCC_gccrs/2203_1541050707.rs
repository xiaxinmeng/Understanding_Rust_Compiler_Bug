rust
trait A {
    fn f(self);
}

impl A for (i32, i32) {
    fn f(self) {}
}
