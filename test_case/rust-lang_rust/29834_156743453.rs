 Rust
struct Foo;

impl Foo {
    fn stuff(self) {}
}

fn compiles() {
    let mut x = None;
    loop {
        x = Some(Foo);
        if let Some(s) = x { s.stuff(); }
    }
}


fn does_not_compile() {
    let mut x = None;
    loop {
        if let Some(s) = x { s.stuff(); } //~ ERROR the type of this value must be known
        x = Some(Foo);
    }
}

fn main() {}
