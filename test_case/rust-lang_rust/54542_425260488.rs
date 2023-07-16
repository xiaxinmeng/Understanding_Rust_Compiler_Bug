rust

use std::mem::MaybeUninit;

fn main() {
    const N: usize = 5;

    let arr: [Vec<u8>; N] = unsafe {
        let mut arr = MaybeUninit::uninitialized();
        for i in 0..N {
            (arr.as_mut_ptr() as *mut Vec<u8>).add(i).write(vec![]);
        }
        arr.into_inner()
    };

    for idx in 0..arr.len() {
        println!("Index {}: {:?}", idx, arr[idx]);
    }
}
