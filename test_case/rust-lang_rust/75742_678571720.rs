rust
struct Foo;
impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        panic!();
    }
}

pub fn foo() {
    format!("{}", Foo);
}
