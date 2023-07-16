rust
#[inline]
fn is_size_suitable(size: usize) -> bool {
    use std::mem;

    let fake_ptr: &$struct_name = unsafe { mem::transmute((0usize, 0usize)) };
    let min_size = mem::size_of_val(fake_ptr);

    let fake_ptr: &$struct_name = unsafe { mem::transmute((0usize, 1usize)) };
    let step = mem::size_of_val(fake_ptr) - min_size;

    size > min_size && (size - min_size) % step == 0
}

