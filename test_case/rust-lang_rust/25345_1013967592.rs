rust
#![feature(decl_macro)]

macro foo {
    () => {
        fn dummy<'a>(x: &'a i8) -> i8 { *x }
    }
}

struct Foo<'a>(&'a i8);

impl<'a> Foo<'a> {
    foo!();
    fn baz() {
        let x = 3_i8;
    }
}

fn main() {}
