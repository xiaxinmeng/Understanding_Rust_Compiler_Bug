rust
struct Type<'a> { a: &'a () }

fn foo(_: &Type<'_>) -> &'_ str { loop {} }

fn main() {}
