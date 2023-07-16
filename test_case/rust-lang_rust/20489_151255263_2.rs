 rust
mod foo {
    pub struct Foo { b: bool }
    pub const V1: Foo = Foo { b: true };
    pub const V2: Foo = Foo { b: false };
}
