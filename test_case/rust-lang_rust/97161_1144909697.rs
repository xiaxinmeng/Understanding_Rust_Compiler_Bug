rust
pub fn use_nonnull1(p: &NonNull) {
    std::hint::black_box(p.pointer);
}

pub fn use_nonnull2(p: &NonNull) {
    let p = *p;
    std::hint::black_box(p.pointer);
}
