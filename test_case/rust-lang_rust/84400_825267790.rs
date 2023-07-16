rust
fn check<S: AsRef<str>>(_: S) -> bool {
    false
}

fn main() {
    let options: Vec<String> = Vec::new();
    let _ = options.into_iter().filter(check::<&String>);
}
