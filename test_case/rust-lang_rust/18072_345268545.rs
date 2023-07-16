rust
pub fn bar(foo: &Fn(&[u8], &[u8], &[u8], &[u8], &[u8], &[u8])) {
    let a = [0u8; 0x7fff_ffff];
    let b = [0u8; 0x7fff_ffff];
    let c = [0u8; 0x7fff_ffff];
    let d = [0u8; 0x7fff_ffff];
    let e = [0u8; 0x7fff_ffff];
    let f = [0u8; 0x7fff_ffff];
    foo(&a, &b, &c, &d, &e, &f);
}
