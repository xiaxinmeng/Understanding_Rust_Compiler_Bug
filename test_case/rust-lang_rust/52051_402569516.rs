rust
unsafe fn swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize) {
    if count == 1 && mem::size_of::<T>() < 32 {
        let z = read(x);
        copy_nonoverlapping(y, x, 1);
        write(y, z);
    } else {
        let x = x as *mut u8;
        let y = y as *mut u8;
        let len = mem::size_of::<T>() * count;
        swap_nonoverlapping_bytes(x, y, len)
    }
}
