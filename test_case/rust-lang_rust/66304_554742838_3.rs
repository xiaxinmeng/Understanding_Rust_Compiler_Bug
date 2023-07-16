
#[test]
fn assert_eq_shell_escape_test() {
    assert_eq!("foo\x1b[1Dbar", "fobar");
}
