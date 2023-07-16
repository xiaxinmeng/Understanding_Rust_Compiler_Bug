rust
#[no_mangle]
extern "C" fn d() {
    struct Double;

    impl Drop for Double {
        fn drop(&mut self) {
            std::process::abort();
        }
    }

    let _d = Double;

    panic!("once");
}
