rust
unsafe {
    let tmp: T = ptr::read(x); // UB if x does not point to valid T
    ptr::write(x, ptr::read(y));
    ptr::write(y, tmp);
}
