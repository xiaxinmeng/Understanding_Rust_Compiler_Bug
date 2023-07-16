rust
#![feature(extern_types)]

extern {
    type foo_t;
    
    fn foo_init(f: *mut foo_t);
}

fn main() {
    let mut a = std::mem::uninitialized();
    unsafe { foo_init(&mut a) };
}
