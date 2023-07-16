 rust
fn f5(s: &mut [isize]) {
    for i in 0 .. s.len() {
        s[i];
        let len = s.len();
        for x in s[i .. len].iter_mut() { *x; }
        for x in s[0 .. i + 1].iter_mut() { *x; }
    }
}
