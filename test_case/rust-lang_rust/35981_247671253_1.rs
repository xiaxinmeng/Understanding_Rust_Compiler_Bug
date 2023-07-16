 rust
fn f5(s: &mut [isize]) {
    for i in 0 .. s.len() {
        s[i];
        for j in i .. s.len() { s[j]; }
        for j in 0 .. i + 1 { s[j]; }
    }
}
