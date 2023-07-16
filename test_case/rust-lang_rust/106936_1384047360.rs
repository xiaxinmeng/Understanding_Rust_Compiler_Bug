rust
enum Void {}
pub enum MyEnum {
    A(u32),
    B(Void, Box<MyEnum>)
}
