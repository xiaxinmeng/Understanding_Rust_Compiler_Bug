rust
fn leak<T>(b: Box<T>) -> &'static mut T {
    unsafe { &mut *Box::into_raw(b) }
}

fn main() {
    let static_ref: &'static mut &mut usize = {
        let mut local_variable = 41;
        let boxed: Box<&mut usize> = Box::new(&mut local_variable);
        leak(boxed)
    };
    **static_ref += 1;
    assert_eq!(**static_ref, 42);
}
