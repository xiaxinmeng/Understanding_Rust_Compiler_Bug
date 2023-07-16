rust
#[test]
fn debugging() {
    std::panic::catch_unwind(|| panic!("hi")).ok();
}
