rust
fn set_len<T>(v: &mut Vec<T>, len: usize) {
    v.len = len;
    if let Some(last) = v.last() {
        let _x: ManuallyDrop<T> = unsafe {
            std::ptr::read(last as *const T as *const ManuallyDrop<T>)
        };
        // put copy of a `T` on the stack, 
        // requires that that value satisfies the language invariants of `T`
    }

