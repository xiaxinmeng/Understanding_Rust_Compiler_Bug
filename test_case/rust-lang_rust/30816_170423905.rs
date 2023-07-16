 rust
extern crate libc;
#[no_mangle]
pub unsafe extern fn memset(mut s: *mut u8, c: libc::c_int, n: usize) -> *mut u8 {
    let end = s.offset(n as isize);
    let c = c as u8;
    while s != end {
        *s = c;
        s = s.offset(1);
    }
    s
}

fn main() {
    let mut arr = [0u8; 32];
    unsafe { memset(&mut arr[0], 1, arr.len()); }
    assert_eq!(arr, [1u8; 32]);
}
