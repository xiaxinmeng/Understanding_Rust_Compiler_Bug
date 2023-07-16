rust
fn _f<T>() {
    extern "C" {
        fn foo(_a: *const T);
    }
}
