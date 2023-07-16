rust
fn main() {
    test().unwrap();
}

fn test() -> Result<(), String> {
    let unit_binding = ();

    Ok(unit_binding)
}
