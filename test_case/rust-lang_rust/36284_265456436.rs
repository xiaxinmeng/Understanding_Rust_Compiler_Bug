rust
pub fn vec_to_ptr_len(v: Vec<u8>) -> (*mut u8, usize) {
    let mut b = v.into_boxed_slice();
    let ptr = b.as_mut_ptr();
    let len = b.len();
    std::mem::forget(b);
    (ptr, len)
}

pub unsafe fn ptr_len_to_vec(ptr: *mut u8, len: usize) -> Vec<u8> {
    Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)).into_vec()
}
