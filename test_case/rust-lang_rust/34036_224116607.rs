 rust
fs::hard_link(src, dst).or_else(|_| fs::copy(src, dst))
