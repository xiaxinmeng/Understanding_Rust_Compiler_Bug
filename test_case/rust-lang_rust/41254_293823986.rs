rust
fn main() {
    let mut s = String::from("foo");
    assert_eq!(&s, "foo");
    s.push_str("bar");
    assert_eq!(&s, "foobar");
    s.push_str("baz");
    assert_ne!(&s, "foobarbar");
}
