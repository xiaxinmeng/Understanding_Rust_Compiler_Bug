rust
fn strip_suffix(input: &str) -> &str {
	input.strip_suffix(char::is_whitespace).unwrap_or(input)
}
