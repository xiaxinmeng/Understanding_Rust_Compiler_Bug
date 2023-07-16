rust
if guard.len() * 3 < guard.capacity() {
    guard.shrink_to_fit();
}
