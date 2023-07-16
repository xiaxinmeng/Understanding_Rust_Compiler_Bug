 rust
pub mod foo {
    pub enum Bar {
        Baz
    }

    fn Bar() { }
}

fn main() {
    use foo::Bar;

    Bar();
}
