 rust
struct MyNewType;
impl Foo for MyNewType {
    type Bar = MyNewType;
}
impl Alternate<MyNewType> for MyNewType {}
