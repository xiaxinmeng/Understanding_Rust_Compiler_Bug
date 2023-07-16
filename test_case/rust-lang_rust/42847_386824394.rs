rust
#![feature(extern_types)]

extern "C" {
    type foo_t;

    fn foo_take_null(f: *mut foo_t);
}

fn main() {
    let a = 0 as *mut foo_t;
    unsafe {
        foo_take_null(a);
    }
}
