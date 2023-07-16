rust
fn a<'a>(v: &'a mut Vec<String>) -> &'a str {
    match v.first::<'a>() {
        Some(s) => return &**s as &'a str,
        None => {}
    }
    // Move this here to try to work around non-lexical borrow
    v.push("".to_owned());
    return "" as &'a str;
}
