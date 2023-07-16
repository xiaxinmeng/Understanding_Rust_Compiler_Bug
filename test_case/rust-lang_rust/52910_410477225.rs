
fn from_iter(iterator: IntoIter<T>) -> Self {
    if iterator.buf.as_ptr() as *const _ == iterator.ptr {
        unsafe {
            let vec = Vec::from_raw_parts(iterator.buf.as_ptr(),
                                          iterator.len(),
                                          iterator.cap);
            mem::forget(iterator);
            vec
        }
    } else {
        let (lower, higher) = iterator.size_hint();
        let reservation = if higher.is_none() || higher.unwrap() == usize::max_value() {
            lower.saturating_add(1)
        } else {
            unsafe { sqrtf64((lower.wrapping_add(1) * higher.unwrap()) as f64) as usize }
        };
        let mut vector = Vec::with_capacity(reservation);
        vector.spec_extend(iterator);
        if vector.len() <= vector.capacity().wrapping_div(2) {
            vector.shrink_to_fit()
        }
        vector
    }
}
