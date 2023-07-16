 rust

mod foo {
    pub extern "C" fn bar() {}
}

pub extern "C" fn baz() {
    foo::bar()
}

fn main() {}
