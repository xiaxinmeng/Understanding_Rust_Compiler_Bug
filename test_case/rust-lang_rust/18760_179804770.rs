 rust
trait A {
    fn b(c: i32);
}

impl A for () {
    fn b(c: ()) {  }
}
