rust
impl MyTrait for MyType {
    type Assoc = impl Trait;

    fn make_foo() {
        let x: Self::Assoc = 22;
        // ...
    }
}
