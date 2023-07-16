 rust
struct FooStruct<'a> {
    val: &'a u8,
}

type Foo<'a> = &'a FooStruct<'a>;
fn foo<'a>(foo: Foo<'a>) { } // could be: fn foo(foo: Foo) { }

fn main() { }
