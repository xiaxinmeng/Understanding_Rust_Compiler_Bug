rust
#[no_mangle]
pub fn method_b(a: u16){
    let b = a as u32;
    assert_eq!(b.wrapping_add(1), b+1);
}
