rust
fn to_c_void_ptr<T>(p: *const T) -> *const libc::c_void {
    p as _
}

fn main() {
    let x : libc::c_int = 4;
    let y : *const libc::c_void = to_c_void_ptr(&x);
}
