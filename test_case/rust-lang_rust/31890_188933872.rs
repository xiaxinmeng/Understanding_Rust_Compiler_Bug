 rust
pub fn imperative_add_vec_vec_zipslices(a: &Vec<i32>, b: &Vec<i32>, c: &mut Vec<i32>) {
    let len = min(a.len(), b.len());
    let len = min(len, c.len());
    let a = &a[..len];
    let b = &b[..len];
    let c = &mut c[..len];
    for i in 0..len {
        c[i] = a[i] + b[i];
    }
}
