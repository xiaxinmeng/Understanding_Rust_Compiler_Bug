rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.pop();
    let ptr = v.as_mut_ptr();
    let ptr = unsafe { &mut *ptr };
    test(&mut v, ptr);
    println!("{:?}", v);    // [8, 2, 3, 4, 7]
}
fn test(v: &mut Vec<u32>, x: &mut u32) {
    v.push(7);
    *x = 8;
}
