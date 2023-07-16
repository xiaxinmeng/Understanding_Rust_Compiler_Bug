 rust
struct Foo;

impl Foo {
    fn save<'a>(&'a self, dest: &mut &'a Foo) -> bool {
        *dest = self;
        true
    }
}

fn main() {
    let mut dest = &Foo;
    if Foo.save(&mut dest) {} else {}
}
