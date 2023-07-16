
#![crate_type = "staticlib"]

#[derive(Copy)]
pub struct Pair {
    foo: u32,
    bar: u32,
}

#[no_mangle]
pub extern fn get_pair() -> Pair {
    Pair {
        foo: 42,
        bar: 10,
    }
}
