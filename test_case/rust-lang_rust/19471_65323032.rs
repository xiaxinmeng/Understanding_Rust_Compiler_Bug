 rust
#[test]
#[should_fail]
fn five_back_failure() {
    assert!( 6i == five_back() || true == true );
}
