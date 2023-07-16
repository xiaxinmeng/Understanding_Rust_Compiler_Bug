 rust
impl Clone for MyEnum {
    fn clone(&self) -> MyEnum {
        match *self {}
    }
}
