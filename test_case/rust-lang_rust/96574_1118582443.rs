rust
#[test]
fn with_panic() {
    let chrome = match headless_chrome::start() {
        Ok(chrome) => chrome,
        Err(err) => test::abort(format!("headless chrome is not available: {err}")),
    }
    // do test
}
