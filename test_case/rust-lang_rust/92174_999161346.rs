rust
pub fn iter_too(v: &[u32]){
    for (i, _) in v.iter().enumerate(){
        assert!(i < v.len());
        assert!(i <= v.len());
    }
}
