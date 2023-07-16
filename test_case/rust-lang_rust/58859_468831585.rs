rust
fn _assert_bounds() {
    fn assert_error<T: Error>() {}

    assert_error::<Box<dyn Error>>();
}
