rust
fn unwraps_or_defaults(s: Option<String>) -> String {
    match s {
        Some(s) => s,
        None => "".into(),
    }
}

fn call() -> String {
   let foo = String::from("foo");
   let foo_or_empty = unwraps_or_defaults(foo.into());
   foo_or_empty
}
