rust
pub fn eq(s1: &S, s2: &S) -> bool {
    (s1.a == s2.a) & (s1.b == s2.b) & (s1.c == s2.c) & (s1.d == s2.d)
}
