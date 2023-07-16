rust
pub fn foo(v: &mut Vec<usize>, k: &mut Vec<usize>) -> usize {
    assert!(v.len() > 2 && k.len() > 2);
    v[0] += 1;
    k[0] += 1;
    v[0] += 1;
    k[0]
}
