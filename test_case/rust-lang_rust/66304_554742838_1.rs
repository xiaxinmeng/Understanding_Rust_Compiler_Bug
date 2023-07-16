
#[test]
#[should_panic(expected = "foo\x1b[1Dbar")]
fn shell_escape_panic_test() {
    panic!("fobar");
}
