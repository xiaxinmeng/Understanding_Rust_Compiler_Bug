rust
#![feature(const_let)]

union Foo<'a> {
    y: &'a (),
    long_live_the_unit: &'static (),
}

const FOO: &() = {
    let y = ();
    unsafe { Foo { y: &y }.long_live_the_unit }
};

fn main() {
}
