
fn lookup_table(key: &str) -> Option<uint> {
    match key {
        "foo" => Some(0),
        "bar" => Some(1),
        "car" => Some(2),
        _ => None
    }
}
