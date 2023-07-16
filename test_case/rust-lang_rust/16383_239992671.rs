 rust
#![feature(asm)]

fn main() {
    struct Descriptor{
        size: usize,
        ptr:  usize,
    }
    let descriptor = Descriptor {
        size: 0,
        ptr:  0,
    };
    unsafe {
        asm!("lidt $0" :: "m"(descriptor) :: "intel");
    }
}
