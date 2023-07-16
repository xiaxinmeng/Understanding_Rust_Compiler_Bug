rust
fn main() {
    unsafe {
        struct S;
        impl S {
            fn f() {
                *std::ptr::null::<u8>(); // unsafe
            }
        }
    }
}
