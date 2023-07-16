 rust
playbot: enum Foo { SingleVariant(i32) } ::std::mem::size_of::<Foo>()
4
playbot: enum Foo { SingleVariant(()) } ::std::mem::size_of::<Foo>()
0
playbot: enum Foo { SingleVariant{} } ::std::mem::size_of::<Foo>()
1
