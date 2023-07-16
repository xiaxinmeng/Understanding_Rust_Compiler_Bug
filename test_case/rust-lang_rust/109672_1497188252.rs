rust
#[inline(never)]
fn double() {
    struct Double;

    impl Drop for Double {
        fn drop(&mut self) {
            std::process::abort();
        }
    }

    let _d = Double;

    panic!("once");
}
