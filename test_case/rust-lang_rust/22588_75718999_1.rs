 rust
let mut s = String::new();
while buf.read_line(&mut s).is_ok() {
    if s.len() == 0 {
        break;
    }
    // code
    s.truncate(0);
}
