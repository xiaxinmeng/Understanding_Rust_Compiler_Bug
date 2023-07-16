rust
fn foo(x: Option<String>) -> String {
    match x {
        Some(s) => s,
        None => unsafe { unreachable_unchecked() }
    }
}
