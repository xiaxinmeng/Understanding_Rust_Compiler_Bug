rust
fn a(v: &mut Vec<String>) -> &str {
    match v.len() {
        0 => {
            v.push("".to_owned());
            ""
        },
        _ => &*v[0]
    }
}
