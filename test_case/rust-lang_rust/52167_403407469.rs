rust
if !run_utf8_validation(v) {
    #[allow(const_err)]
    let _ = std::u32::MAX + 1;
}
