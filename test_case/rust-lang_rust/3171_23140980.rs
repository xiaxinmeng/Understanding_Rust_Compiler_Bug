 rust
fn with_type_params<T>() { }

extern fn foo<T>() {
    with_type_params::<T>();
}

fn main() {
    let _a:*u8 = foo;
}
