
#[no_mangle]
pub unsafe extern "C" fn(cb: extern "C" fn([u8; 32])) { // WRONG - should be fn(*const [u8; 32])
    let arr = [8u8; 32];
    cb(arr); // WRONG - should be &arr
}
