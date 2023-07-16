rust
pub fn got() -> Vec<u32> {
    let mut res = Vec::new();
    let s1 = vec![1, 2, 3, 4];
    res.extend_from_slice(&s1); let s1 = s1;
    res.extend_from_slice(&s1); let s1 = s1;
    res.extend_from_slice(&s1); let s1 = s1;
    res.extend_from_slice(&s1);
    res
}

pub fn expect() -> Vec<u32> {
    let mut res = Vec::new();
    let s1 = vec![1, 2, 3, 4];
    res.extend_from_slice(&s1);
    res.extend_from_slice(&s1);
    res.extend_from_slice(&s1);
    res.extend_from_slice(&s1);
    res
}
