rust
loop {
    x = intrinsics::align_offset(ptr, align);
    if (x is aligned to object start) { return }
    ptr = x.offset(1);
}
