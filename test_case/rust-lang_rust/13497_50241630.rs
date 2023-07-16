
fn read_lines_borrowed() -> Vec<&str> {
    let rawLines: Vec<String> = vec!["foo  ".to_string(), "  bar".to_string()];
    rawLines.iter().map(|l| l.as_slice().trim()).collect()
}
