rust
pub const fn check_bag<T: Copy>(bag: &BagOfBits<T>) {
    let val = unsafe { *(bag as *const _ as *const u8) };
    assert!(val == 1);
}

const _: () = check_bag(&make_1u8_bag::<usize>());
