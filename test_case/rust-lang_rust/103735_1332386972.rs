rust
fn main() {
    let mut b = Box::new(Some(0usize));
    let raw = Box::into_raw(b);
    unsafe {
        let r = &*raw;
        let ptr = raw as *const Option<usize>;
        let new_box = Box::from_raw(ptr as *mut usize);
        let z = (*ptr).is_none();
        drop(new_box); // Call drop explicitly to make the error simpler
    }
}
