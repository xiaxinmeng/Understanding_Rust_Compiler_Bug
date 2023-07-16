 rust
try {
    drop_in_place(a);
} finally {
    ptr::write(a, b);
}
