rust
fn foo(x: Option<String>) {
  match x {
    Some(mut ref s) if s.starts_with("hello") => s.push_str(" world!"),
    _ => {},
  }
}
