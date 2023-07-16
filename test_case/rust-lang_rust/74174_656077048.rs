rust
#![allow(dead_code)]
use std::marker::PhantomData;

struct Slice<'a, T: 'a> {
    start: *const T,
    // end: *const T,
    phantom: PhantomData<&'a T>,
}

fn borrow_vec<T>(vec: &Vec<T>) -> Slice<'_, T> {
    let ptr = vec.as_ptr();
    Slice {
        start: ptr,
        // end: unsafe { ptr.add(vec.len()) },
        phantom: PhantomData,
    }
}

fn f1() {
    let sl;
    {
        let vec = vec![1];
        sl = borrow_vec(&vec);
    }
    let __ = sl;
}

fn f2() {
    let sl;
    {
        let vec = vec![1];
        // let ptr = vec.as_ptr();
        sl = Slice {
            start: &vec,
            // end: unsafe { ptr.add(vec.len()) },
            phantom: PhantomData,
        };
    }
    let __ = sl;
}

fn main() {}
