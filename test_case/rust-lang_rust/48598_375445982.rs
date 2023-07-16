rust
struct Foo<'a> {
    x: &'a u32,
}

struct Bar<'a, 'b: 'a> {
    x: &'a mut Foo<'b>
}

impl<'a> Foo<'a> {
    fn method(&mut self, value: &u32) {
        
    }
}

fn trick1() {
    let mut f = &mut Foo { x: &22 };
    let z = Bar {x: f};
    // This is hard because we need to look "through" a projection to figure out where the borrow came from
    Foo::method(z.x, z.x.x)
}

fn trick2() {
    let mut b = &mut Foo { x: &7 };
    let mb = b;
    // similarly, there is more than one assignment we would have to consider
    Foo::method(mb, mb.x);
}
