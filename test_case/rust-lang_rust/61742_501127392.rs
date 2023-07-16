rust
if let Err(pos) = s.binary_search(&num) {
    s.insert(pos, num);
}
