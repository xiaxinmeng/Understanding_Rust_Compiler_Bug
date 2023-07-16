rust
if let Some(s) = c.try_encode_utf8(buf) {
    // work with str
} else {
    // alternate action, but cannot access `buf` here until nll are a thing
}
