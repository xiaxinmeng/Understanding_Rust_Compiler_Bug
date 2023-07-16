rust
fn miri_issue_2759() {
    let mut input = "1".to_string();
    input.replace_range(0..0, "0");
}
