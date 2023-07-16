 rust
#![feature(linkage)]

#[link_name = "c_part"]
extern {
    #[linkage = "extern_weak"] static A: *const usize;
    fn f();
}

fn main() {
    unsafe {
        f();
        println!("{:x} @ {:x} @ {:p}", *(*A as *const usize), *A, A);
    }
}
