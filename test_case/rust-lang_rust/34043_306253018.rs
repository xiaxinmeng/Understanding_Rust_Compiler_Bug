rust
#![feature(asm, naked_functions)]

#[naked]
#[no_mangle]
pub unsafe extern "C" fn naked(_arg: u64) {
    asm!("ret" :::: "intel");
}

#[no_mangle]
pub extern "C" fn non_naked(_arg: u64) {
    return;
}
