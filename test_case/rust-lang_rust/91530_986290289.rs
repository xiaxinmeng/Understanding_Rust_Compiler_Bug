rust
if code.starts_with('(') && code.ends_with(')') {
    <suggest adding a comma at span.with_hi(span.hi - 1)>
} else {
    <multipart suggestion>
}
