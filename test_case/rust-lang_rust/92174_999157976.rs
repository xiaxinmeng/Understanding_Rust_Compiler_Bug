rust
pub fn iter_too(v: &[u32]){
    for (_, i) in v.iter().zip(0..v.len()) {
        assert!(i <= v.len());
    }
}
