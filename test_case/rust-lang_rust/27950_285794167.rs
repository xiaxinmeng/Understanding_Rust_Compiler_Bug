rust
#[derive(PartialEq)]
struct Foo<'a, A, B> {
    a: &'a [A],
    b: Vec<B>,
    c: String,
}

// generated
impl<'a1, 'a2, A1, A2, B1, B2> PartialEq<Foo<'a2, A2, B2>> for Foo<'a1, A1, B1>
where
    &'a1 [A1]: PartialEq<&'a2 [A2]>, // for self.a == other.a
    Vec<B1>: PartialEq<Vec<B2>>,     // for self.b == other.b
    String: PartialEq<String>,       // for self.c == other.c. Obviously redundant, but not a real problem and the macro can't be type-directed.
{
    ...
}
