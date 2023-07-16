rust
fn main() {
    let _ = foo(Some("I'm a string"));
    let _ = foo(None);  // ERROR: type annotation needed
}

pub fn foo<S = String>(opt: Option<S>) -> String
    where S: Into<String>
{
    opt.map(Into::into).unwrap_or_default()
}
