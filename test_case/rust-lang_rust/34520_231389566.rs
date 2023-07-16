
fn main() {
    process::exit(foo() as i32);
}
fn foo() -> u8 {
    let mut x = UnsafeCell::new(1);
    let px = x.get();
    bar(&mut x, px);
    unsafe { x.into_inner() }
}
fn bar(v1: &mut UnsafeCell<u8>, v2: *mut u8) {
    unsafe {
        let a = *v1.get();
        *v2 = 3;
        *v1.get() = a;
    }
}
