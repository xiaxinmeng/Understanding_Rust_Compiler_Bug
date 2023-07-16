rust
enum IsInFollow {
    Yes,
    No(&'static [&'static str]),
    Invalid(String, &'static str),
}
