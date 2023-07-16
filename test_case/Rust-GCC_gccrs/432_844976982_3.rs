rust=
mod foomod {
    struct Foo {
     f: i32,
     g: u32,
    }
}

fn test() -> foomod::Foo {
    foomod::Foo{
        f:1,
        g:3,
    }
}
