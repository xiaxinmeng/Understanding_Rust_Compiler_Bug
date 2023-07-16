
pub mod foo {
    enum Bar {
        Baz
    }

    pub fn Bar() { }
}

fn main() {
    use foo::Bar;

    Bar();
}
