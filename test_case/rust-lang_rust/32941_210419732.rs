 rust
fn main() {
    use std::ascii::AsciiExt;
    let mut s = "Per Martin-Löf".to_string();

    {
        let (first, last) = s.split_at_mut(3);

        first.make_ascii_uppercase();
        last.make_ascii_lowercase();

        assert_eq!("PER", first);
        assert_eq!(" martin-löf", last);
    }

    assert_eq!("PER martin-löf", s)
}
