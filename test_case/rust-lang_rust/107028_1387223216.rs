
#![feature(unsize)]
#![feature(ptr_metadata)]

use std::marker::PhantomData;
use std::marker::Unsize;
use std::mem;
use std::ptr;
use std::ptr::NonNull;
use std::ptr::Pointee;

const fn zst_metadata<U: ?Sized, S: Unsize<U>>() -> <U as Pointee>::Metadata {
    ptr::metadata(NonNull::<S>::dangling().as_ptr() as *const U)
}

struct GetMetadata<U: ?Sized, S: Unsize<U>>(PhantomData<fn(&U, &S)>);

impl<U: ?Sized, S: Unsize<U>> GetMetadata<U, S> {
    const METADATA: <U as Pointee>::Metadata = zst_metadata::<U, S>();
}

fn thin_box_new_unsized<U: ?Sized, S: Unsize<U>>(s: &S) -> *mut u8 {
    if mem::size_of_val(s) == 0 {
        unsafe {
            ((&GetMetadata::<U, S>::METADATA) as *const <U as Pointee>::Metadata).add(1) as *mut u8
        }
    } else {
        panic!("regular ThinBox")
    }
}
