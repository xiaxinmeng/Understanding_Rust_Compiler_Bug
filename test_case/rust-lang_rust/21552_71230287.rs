 rust
struct Foo<'a> {
    r: &'a mut u32
}

impl<'a> Foo<'a> {
    fn get(&'a self) -> &'a u32 {
        &*self.r
    }
}

fn foo<'x>(foo: Foo<'x>) -> &'x int {
    foo.get() // ERROR
}

fn main() {
    let f = Foo { r: &mut 0 };
    foo(f);
}
