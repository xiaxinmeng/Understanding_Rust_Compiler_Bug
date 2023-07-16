rust
struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("hello!");
    }
}

fn main() {
    let _a = A;
    cause_a_segfault();
}

fn cause_a_segfault() {
    unsafe { *(0x1 as *mut u32) = 3; }
}
