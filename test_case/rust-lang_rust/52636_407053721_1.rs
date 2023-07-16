rust
use std::arch::x86_64;

#[repr(align(32))]
struct BytePair {
    load_bytes: [u8; 32],
    store_bytes: [u8; 32]
}

fn main() {
    let mut byte_pair = BytePair{
        load_bytes: [0x0f; 32],
        store_bytes: [0; 32]
    };
    let lb_ptr = byte_pair.load_bytes.as_ptr();
    let reg_load = unsafe {
        x86_64::_mm256_load_si256(
            lb_ptr as *const x86_64::__m256i
        )
    };
    println!("{:?}", reg_load);
    let sb_ptr = byte_pair.store_bytes.as_mut_ptr();
    unsafe {
        x86_64::_mm256_store_si256(sb_ptr as *mut x86_64::__m256i, reg_load);
    }
    assert_eq!(&byte_pair.load_bytes, &byte_pair.store_bytes);
}
