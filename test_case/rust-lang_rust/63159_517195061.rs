rust
fn copy<T>(src: *const T, dest: *mut T, len: usize) {
    for i in 0..len { dest.add(i).write(src.add(i).read()) } // EDIT: see rkruppe below
}
