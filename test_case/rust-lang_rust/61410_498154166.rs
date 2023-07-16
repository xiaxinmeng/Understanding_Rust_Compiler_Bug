
unsafe fn generate<T, const SIZE: usize>(mut next_val: impl FnMut() -> T) -> [T; SIZE] {
    let mut arr: [T; SIZE] = std::mem::uninitialized();

    for el in arr.iter_mut() {
        let mut data = next_val();

        std::ptr::write(el, data);
    }

    arr
}
