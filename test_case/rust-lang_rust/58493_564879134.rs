
pub trait T {
    extern fn hello_rust();
}

struct S;

impl T for S {
    #[no_mangle]
    extern fn hello_rust() {
    }
}
