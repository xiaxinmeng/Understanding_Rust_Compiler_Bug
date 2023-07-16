rust
if buf.len() >= c.utf8_len() {
    let s = c.encode_utf8(buf);
    // work with str
} else {
    // alternate action
}
