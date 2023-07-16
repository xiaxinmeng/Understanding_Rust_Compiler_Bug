
struct Foo<'self> {
    field1: &'self Bar
    field2: Option<&'self Baz>,
}
