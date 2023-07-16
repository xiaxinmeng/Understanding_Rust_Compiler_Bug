 rust
impl<T> Vec<T> {
    pub type raw = VecRawConstructor<T>;
    // ...
}

pub struct VecRawConstructor<T>;

impl<T> VecRawConstructor<T> {
    pub fn from_parts(&self, length: uint, capacity: uint, ptr: *mut T) -> Vec<T> { ... }
}

fn example_usage() {
    let (len, cap, ptr) = ...;
    let v = unsafe { Vec::raw::from_parts(len, cap, ptr) };
    // ...
}
