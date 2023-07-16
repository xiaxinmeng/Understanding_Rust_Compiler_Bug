 rust
struct Inner<'a> { s: &'a str }

struct Outer<'a>(Inner<'a>);
