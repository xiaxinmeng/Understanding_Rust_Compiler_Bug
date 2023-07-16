rust
pub fn foo2(s: &[u8]) -> &[u8] {
    let mut idx = 0;
    while idx < s.len() {
        if s[idx] == b'\\' {
            return &s[.. idx];
        }
        idx += 1;
    }
    s
}
