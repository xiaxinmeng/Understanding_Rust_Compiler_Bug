 rust
fn f5(s: &mut [isize]) {
    for i in 0 .. s.len() {
        s[i];
        let len = s.len();
        &s[i .. len];
        &s[0 .. i + 1];
    }
}
