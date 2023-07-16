rust
#![feature(linkage)]

fn main() {
    extern {
        #[linkage = "weak"]
        static A: *const u8;
    }
    {
        #[no_mangle]
        static A: u8 = 1;
    }
    unsafe { assert_eq!(*A, 1); }
}
