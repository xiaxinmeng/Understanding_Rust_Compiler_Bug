rust
#![feature(const_refs_to_cell)]
#![feature(const_ptr_read)]
#![feature(const_ptr_offset)]
#![feature(generic_const_exprs)]

use core::mem::ManuallyDrop;

const fn split_arr<T, const N: usize, const M: usize>(arr: [T; N]) -> ([T; M], [T; N - M])
where
    [T; N - M]: ,
{
    let arr = ManuallyDrop::new(arr);
    let start_ptr = &arr as *const _ as *const T;
    let arr_1_ptr = start_ptr as *const [T; M];
    let arr_2_ptr = unsafe { start_ptr.add(M) } as *const [T; N - M];

    unsafe { (arr_1_ptr.read(), arr_2_ptr.read()) }
}

const ARR: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

const FIRST_3: [u8; 3] = split_arr(ARR).0;

// This last one doesn't work, since rustc can't infer the constant
// const LAST_7: [u8; 7] = split_arr(ARR).1;

// We just need to specify the generic params...
const LAST_7: [u8; 7] = split_arr::<u8, 10, 3>(ARR).1;
