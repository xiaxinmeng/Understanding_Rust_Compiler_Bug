rust
pub fn alignment_check() {
    println!("Alignment of u64: {}", core::mem::align_of::<u64>());
    let single_u64 = 443u64;
    println!("Address of single_u64: {:08x}", &single_u64 as *const u64 as usize);
}
