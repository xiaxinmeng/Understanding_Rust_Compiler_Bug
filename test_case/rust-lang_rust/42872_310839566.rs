rust
let mut x = Cow::Borrowed("hello world");
x.to_mut().make_ascii_lowercase();
assert_eq!(Cow::Owned("hello world"), x);
