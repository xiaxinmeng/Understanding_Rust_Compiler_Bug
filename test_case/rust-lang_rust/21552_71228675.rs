 rust
struct Foo<'a> {
    r: &'a u32
}

impl<'a> Foo<'a> {
    fn get<'b>(&'b self) -> &'a u32 {
        &*self.r
    }
}

fn foo<'x>(foo: Foo<'x>) {
    foo.get();
}

fn main() {
    let f = Foo { r: &mut 0 };
    foo(f);
}
