rust
unsafe fn setup_boostrap_code<A>() {
    extern "C" {
        static xxx:  *const A;
    }

    let arg1_pointer_new: *const u64 = core::mem::transmute(&xxx);
}
