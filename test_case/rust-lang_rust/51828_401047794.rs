rust
#![feature(const_fn, untagged_unions)]

use std::mem;

union Realign<T> {
    data: T,
    #[cfg(target_os = "macos")]
    other: [u8; mem::size_of::<T>() + mem::align_of::<T>()],
}

impl<T> Realign<T> {
    fn get(me: *const Realign<T>) -> *const T {
        unsafe {
            let data = &(*me).data as *const T;

            #[cfg(target_os = "macos")]
            {
                let align = mem::align_of::<T>();
                if (data as usize) & (align - 1) != 0 {
                    let start = (*me).other.as_ptr() as usize;
                    let ptr = (start + align - 1) & (align - 1);
                    let end = start + (*me).other.len();
                    assert!(start <= ptr && ptr + mem::size_of::<T>() < end);
                    return ptr as *const T
                }
            }
            return data
        }
    }
}
