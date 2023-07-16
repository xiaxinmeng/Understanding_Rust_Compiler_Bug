
pub unsafe fn cas_tagged(ptr: *const u64, old: (u64, u64), new: u64) -> (u64, u64) {
    let mut val: u64 = old.0;
    let mut counter: u64 = old.1;
    let ncounter: u64 = counter.wrapping_add(1);
    asm!("lock cmpxchg16b ($6)\n\t
          jne .Lfail\n\t
          mov $4, $0\n\t
          mov $5, $1\n\t
          .Lfail:"
         : "={rax}"(val), "={rdx}" (counter)
         : "0"(val), "1"(counter), "r"(new), "r"(ncounter), "r"(ptr)
         : "memory"
         : "volatile");
    (val, counter)
}
