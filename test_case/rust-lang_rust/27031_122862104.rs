 Rust
struct Foo3<'a> {
    a: &'a mut Fn(&'static str) -> u32 + 'a
}

fn foo3<'a : 'b, 'b>(foo: Foo3<'a>) -> Foo3<'b> {
    match foo { Foo3 { a } => Foo3 { a: a }} 
}

fn main() {}
