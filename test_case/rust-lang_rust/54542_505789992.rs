rust
use std::mem::{self, MaybeUninit};
use std::ptr;

let data = {
    // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
    // safe because the type we are claiming to have initialized here is a
    // bunch of `MaybeUninit`s, which do not require initialization.
    let mut data: [MaybeUninit<Vec<u32>>; 1000] = unsafe {
        MaybeUninit::uninit().assume_init()
    };

    // Dropping a `MaybeUninit` does nothing, so if there is a panic during this loop,
    // we have a memory leak, but there is no memory safety issue.
    for elem in &mut data[..] {
        unsafe { ptr::write(elem.as_mut_ptr(), vec![42]); }
    }

    // Everything is initialized. Transmute the array to the
    // initialized type.
    unsafe { mem::transmute::<_, [Vec<u32>; 1000]>(data) }
};

assert_eq!(&data[0], &[42]);
