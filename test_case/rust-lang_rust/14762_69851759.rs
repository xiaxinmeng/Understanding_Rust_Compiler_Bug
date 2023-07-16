 rust
#![feature(lang_items, intrinsics)]
#![crate_type="rlib"]
#![no_std]

#[lang="copy"]
trait Copy {}
#[lang="sized"]
trait Sizes {}

extern "rust-intrinsic" {


    /// Abort the execution of the process.
    pub fn abort() -> !;
}
#[lang="panic_bounds_check"]
fn panic_bounds_check(file_line: &(&'static str, usize),
                      index: usize, len: usize) -> ! {
    unsafe {
        abort();
    }
}

extern "C" {
    fn puts(p: *const i8) -> i32;
}

unsafe fn dostuff(p: *const u8) {
    puts(p as *const i8);
}

pub fn main() {
    unsafe { dostuff((&b"TEXT!"[0]) as *const u8); }
}
