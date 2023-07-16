 Rust
mod foo {
    // Private implementation detail struct
    struct Strut;
    fn doit() {
        self::bar::fun();
    }
    mod bar {
        // bar::fun() is a private implementation detail of mod foo
        pub fn fun() -> super::Strut {
            unimplemented!()
        }
    }
}

fn main() {
    // foo::bar::fun(); // inaccessible outside of `foo`
}
