rs
pub fn disco(foo: Foo) -> std::mem::Discriminant<Foo> {
    std::mem::discriminant(&foo)
}
