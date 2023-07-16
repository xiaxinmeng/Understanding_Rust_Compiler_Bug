rust
#![feature(extern_types)]
#![feature(unsized_locals)]

extern {
    type Foo;
}

fn mk_foo() -> Box<Foo> {
    unimplemented!()
}

fn unsized_local_generic_parameter<T: ?Sized>(x: Box<T>) {
    // `unsized_locals` allows you to create locals of generic `?Sized`
    // parameter types
    let _x = *x;
}

fn main() {
    // But `extern_types` allows you to create pointers to extern types, and
    // pass them to functions expecting a pointer to a `?Sized` parameter type
    unsized_local_generic_parameter(mk_foo());
}
