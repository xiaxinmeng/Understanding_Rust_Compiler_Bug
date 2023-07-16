 rust
#![feature(used)]

mod foo {
    // private and non-reachable
    #[no_mangle]
    #[used]
    static STATIC: [u32; 10] = [1; 10];
}
