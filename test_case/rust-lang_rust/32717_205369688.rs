 rust
unsafe {
    s.truncate(10); // panics if s isn't long enough.
    s.as_mut_vec().as_mut_ptr() = b"1234567890";
}
