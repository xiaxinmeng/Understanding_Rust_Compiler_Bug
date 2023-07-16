rust
fn ptr_after<T>(x: &T) -> *const T {
    (x as *const T).offset(1)  // Ok
}

fn ptr_after2<T>(x: &T) -> *const T {
    x.offset(1)
}
