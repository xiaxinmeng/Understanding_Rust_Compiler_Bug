 Rust
if cfg!(target_pointer_width = "64") {
    assert_eq!(size_of::<[u8; (1 << 32)]>(), (1 << 32));
}
