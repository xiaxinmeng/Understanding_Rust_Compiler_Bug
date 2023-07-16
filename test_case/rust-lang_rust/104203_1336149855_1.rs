rust
#![feature(const_slice_split_at_not_mut)]
#![feature(const_pointer_is_aligned)]
#![feature(pointer_is_aligned)]

use core::slice;

const fn bad_align_to(slice: &[u8]) -> (&[u8], &[u16], &[u8]) {
    if slice.is_empty() {
        return (&[], &[], &[]);
    }

    let (front, aligned) = if slice.as_ptr().is_aligned_to(2) {
        ([].as_slice(), slice)
    } else {
        slice.split_at(1)
    };
    
    let (middle, back) = aligned.split_at(aligned.len() / 2 * 2);
    
    // SAFETY: It's safe to transmute between `[u8; N * 2]` and `[u16; N]`.
    // We ensure that `middle` is aligned to 2 above.
    let middle_transmute = unsafe {
        slice::from_raw_parts(middle.as_ptr().cast::<u16>(), middle.len() / 2)
    };
    
    (front, middle_transmute, back)
}

fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    for start in 0..a.len() {
        for end in start..a.len() {
            let slice = &a[start..end];
            println!("{:X?} -> {:X?}", slice, bad_align_to(slice));
        }
    }
    
    const _FOO: &[u16] = bad_align_to(&[1, 2, 3]).1; // ERROR
    const _BAR: u16 = bad_align_to(&[1, 2, 3]).1[0]; // ERROR after #104616
}
