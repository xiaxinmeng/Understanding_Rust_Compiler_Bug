 rust
struct Foo<'a> {
    r: &'a mut u32
}

impl<'a> Foo<'a> {
    fn get(&'a mut self) -> &'a mut u32 {
        &mut *self.r
    }
}

fn foo<'x>(foo: Foo<'x>) {
    foo.get(); // ERROR
}

fn main() {
    let f = Foo { r: &mut 0 };
    foo(f);
}
