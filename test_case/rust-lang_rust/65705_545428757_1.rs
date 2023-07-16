rust
let mut v: Vec<Foobar<T>> = unimplemented!();

let values = unsafe {
    let data = v.as_mut_ptr() as *mut T;
    let len = v.len();
    let cap = v.capacity();

    std::mem::forget(v);

    Vec::from_raw_parts(data, len, cap)
};
