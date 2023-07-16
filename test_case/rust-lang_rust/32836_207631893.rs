
union Foo {
    f: i32,
    g: Box<i32>,
}

let mut f = Foo { g: Box::new(1) };
f.g = Box::new(2);
