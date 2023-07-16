rust
// This strange test is (sometimes?) causing memory corruption within
// the compiler itself.

#![feature(linkage)]

mod dep1 {
    extern {
        #[linkage="external"]
        pub static collision: *const i32;
    }
}

#[no_mangle]
pub static collision: usize = 0;

fn main() {
    unsafe {
       println!("{:p}", &dep1::collision);
    }
}
