
fn expected_error_message(b: u8) {
    match b {
        1 => {}
        0 | 5..=u8::MAX => {}
    }
}
