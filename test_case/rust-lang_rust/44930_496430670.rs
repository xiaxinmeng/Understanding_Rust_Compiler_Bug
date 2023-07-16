
#[repr(C)]
struct Foo {
    x: i32
}

#[repr(C)]
struct Bar {
    foo: Foo
}


unsafe extern fn vararg_test(n: usize, mut args: ...) {
    ...
    args.arg::<Bar>(); //this cannot be compiled
    ...
}
