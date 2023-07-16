
#[derive(Debug)]
#[repr(C)]
struct Foo {
    bar: u32,
}

let foo = Foo { bar: 0 };

let a: &Debug = &foo;
let b: &Debug = &foo.bar;
