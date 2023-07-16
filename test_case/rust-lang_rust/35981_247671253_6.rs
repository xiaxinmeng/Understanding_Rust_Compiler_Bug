 rust
fn obtuse(s: &mut [isize]) {
    if s.len() >= 2 {
        for i in 0 .. s.len() {
            if i < s.len() - 2 {
                &s[0 .. i + 2];
            }
        }
    }
}
