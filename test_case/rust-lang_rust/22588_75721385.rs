 rust
let mut s = String::new();
while { s.truncate(0); buf.read_line(&mut s).is_ok() && s.len() > 0 } {
}
