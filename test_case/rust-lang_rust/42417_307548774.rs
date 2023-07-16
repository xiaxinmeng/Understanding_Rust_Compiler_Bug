rust
fn bar() -> &'static str { "bar" }

fn main() {
    let s = String::from("foo");
    Some(&s[..]).unwrap_or_else(bar);
}
