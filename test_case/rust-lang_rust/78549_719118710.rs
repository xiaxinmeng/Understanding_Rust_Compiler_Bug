rust
fn main() {
    match "foo" {
        "foo" => {},
        &_ => {},
    }
}
