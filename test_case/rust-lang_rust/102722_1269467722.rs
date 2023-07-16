rs
unsafe fn read_u32_unaligned(ptr: *const [u8]) -> u32 {
    *ptr.cast()
}

fn main() {
    let x: Box<[u8; 8]> = Box::new([1,2,3,4,5,6,7,8]);

    unsafe {
        println!("{:08X}", read_u32_unaligned(&*x));
        println!("{:08X}", read_u32_unaligned(&x[1..])); // at least one of these accesses is definitely unaligned
    }
}
