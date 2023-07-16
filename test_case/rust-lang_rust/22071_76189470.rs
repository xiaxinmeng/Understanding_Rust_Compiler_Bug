 rust

#[no_mangle]
pub unsafe extern "C" fn fn_with_c_abi(s: *const u8, len: i32) -> i32 {
    let local0 = len - 1;
    let local1 = len > 2;
    let local2 = (len as f64) + 0.5;

    zzz(); // #break

    return 0;
}

fn main() {
    unsafe {
        fn_with_c_abi(b"abcd\0".as_ptr(), 20);
    }
}

#[inline(never)]
fn zzz() {()}
