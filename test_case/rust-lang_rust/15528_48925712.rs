 rust
mod a {
    pub struct Foo;
    impl Foo { pub fn bar() {} }
}

fn main() {
    use a::Foo::bar;    // error: Cannot import from a trait or type implementation
    bar();
}
