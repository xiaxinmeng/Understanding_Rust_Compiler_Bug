rust
let v: Vec<Foobar<T>> = unimplemented!();

let values = unsafe {
    let (data, len, cap) = v.into_raw_parts();
    Vec::from_raw_parts(data as *mut T, len, cap)
};
