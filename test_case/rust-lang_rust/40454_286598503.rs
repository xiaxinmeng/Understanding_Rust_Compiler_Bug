rust
fn swap_u64x4<T>(x: &mut T, y: &mut T) {
    unsafe {
        #[allow(non_camel_case_types)]
        #[repr(simd)]
        struct u64x4(u64, u64, u64, u64);

        let mut t: u64x4 = mem::uninitialized();

        let x = x as *mut T as *mut u8; 
        let y = y as *mut T as *mut u8; 
        let t = &mut t as *mut _ as *mut u8; 

        let len = mem::size_of::<T>() as isize;
        let block_sz = mem::size_of::<u64x4>();

        let mut i = 0;
        while i + block_sz as isize <= len {
            ptr::copy_nonoverlapping(x.offset(i), t, block_sz);
            ptr::copy_nonoverlapping(y.offset(i), x.offset(i), block_sz);
            ptr::copy_nonoverlapping(t, y.offset(i), block_sz);
            i += block_sz as isize;
        }
        if i < len {
            let rem = (len - i) as usize;
            ptr::copy_nonoverlapping(x.offset(i), t, rem);
            ptr::copy_nonoverlapping(y.offset(i), x.offset(i), rem);
            ptr::copy_nonoverlapping(t, y.offset(i), rem);
        }   
    }   
}

#[no_mangle]
pub fn swap_array_u64x4(x: &mut [u8; 2048], y: &mut [u8; 2048]) {
    swap_u64x4(x, y)
}
