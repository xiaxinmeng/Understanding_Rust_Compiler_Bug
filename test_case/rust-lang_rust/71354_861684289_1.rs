rust
#[inline(never)]
fn black_box<T>(x: T) {
    unsafe { std::ptr::read_volatile(&x); }
}

pub fn foo() -> usize {
    let v = vec![1];
    black_box(&v);
    v[0]
}
