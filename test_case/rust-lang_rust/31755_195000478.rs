 rust
let v = Vec::with_capacity(slice.len());
unsafe {
    v.set_len(slice.len());
}
v.copy_from_slice(slice);
