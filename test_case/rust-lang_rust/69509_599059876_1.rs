rust
pub fn repeat_take_collect() -> Vec<u8> { unsafe {
    let iterator = iter::repeat(42).take(100000);
    let mut v = Vec::new();
    let (low, high) = iterator.size_hint();
    if let Some(additional) = high {
        v.reserve(additional);
        let mut ptr = v.as_mut_ptr().add(v.len());
        iterator.for_each(move |el| {
            ptr::write(ptr, el);
            ptr = ptr.offset(1);
        });
        v.set_len(v.len() + additional);
    }
    v
} }
