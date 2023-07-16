
struct Foo;

impl Foo {
    fn borrow_mut(&mut self) -> Option<&str> {
        Some("Hello World")
    }
}

fn main() {
    let mut foo = Foo;
    let mut retval = None;
    while retval.is_none() {
        retval = foo.borrow_mut()
    };
}
