rust
#![feature(extern_types)]

extern "C" {
    type foo_t;

    fn foo_init(f: *mut *mut foo_t);
}

fn main() {
    unsafe {
        let mut a = std::ptr::null_mut();
        foo_init(&mut a);
    }
}
