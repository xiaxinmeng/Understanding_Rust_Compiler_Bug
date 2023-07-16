rust
#[cfg(feature = "implicit_map")]
#[test]
fn unwrap_option() {
    assert_eq!(try_match!(Some(a) = Some(42)), Ok(42));
}
