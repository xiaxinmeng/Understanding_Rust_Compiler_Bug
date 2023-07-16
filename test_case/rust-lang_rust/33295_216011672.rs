 rust
pub struct Foo<'a> {
    x: fn(&'a i32)
}

pub fn foo<'a>() -> Foo<'a> {
    Foo {
        x: the_fun as fn(&'a i32)
    }
}

fn the_fun<'a>(_: &'a i32) { }
