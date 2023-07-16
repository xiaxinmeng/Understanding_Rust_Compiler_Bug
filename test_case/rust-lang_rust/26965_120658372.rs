 rust
for byte in self.to_bytes().iter().flat_map(|b| ascii::escape_default(*b)) {
    write!(f, "{}", byte as char);
}
