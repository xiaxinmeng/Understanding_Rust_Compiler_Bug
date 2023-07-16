rust
let mut x = Cow::Borrowed("hello world");
x.make_ascii_lowercase();
assert_eq!(Cow::Borrowed("hello world"), x);
