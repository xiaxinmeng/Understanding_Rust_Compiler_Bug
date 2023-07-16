rust
#![feature(asm)]

#[repr(C)]
struct MyPtr(usize);

fn main() {
    unsafe {
        let target = MyPtr(0);
        asm!( "ret" : : "{rdi}"(target) );
    }
}
