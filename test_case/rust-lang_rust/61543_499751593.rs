rust
enum IsInFollow {
    Yes,
    No(Vec<&'static str>),
    Invalid(String, &'static str),
}
