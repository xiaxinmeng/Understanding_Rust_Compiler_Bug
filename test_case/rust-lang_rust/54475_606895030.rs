rust
pub fn a() -> usize {
    let v: Vec<u8> = Vec::new(); 
    let mut v2: Vec<u8> = Vec::new();

    if v.len() > 1 { // false
        v2 = vec![2];
    } else { 
        const NEW: Vec<u8> = Vec::new();
        v2 = NEW;
    }
    v.len() + v2.len() // 0
}
