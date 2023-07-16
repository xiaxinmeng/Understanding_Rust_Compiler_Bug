rust
//@only-32bit
//@only-host-x86_64
fn main() {
    for _ in 0..4 {
        let mut a: Vec<u8> = Vec::with_capacity(1024 * 1024 * 1024);
        let _ptr = a.spare_capacity_mut().as_ptr();
    }
}
