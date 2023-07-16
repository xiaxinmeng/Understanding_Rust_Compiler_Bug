rust
fn foo(s: &str) -> impl Sized + '_ {
    move |()| s.chars().map(|c| format!("{}{}", c, s))
}
