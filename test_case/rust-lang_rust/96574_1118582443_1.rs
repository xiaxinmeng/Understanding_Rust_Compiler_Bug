rust
#[test]
#[ignore(if = headless_chrome::exists, reason = "headless chrome is not available")]
fn with_if() {
    let chrome = match headless_chrome::start() {
        Ok(chrome) => chrome,
        Err(err) => panic!("headless chrome is not available: {err}"),
    }
    // do test
}
