
use std::ops::Deref;

struct Test1<'a, T: ?Sized> {
    cell: &'a core::cell::Cell<usize>,
    data: *const T,
}
struct Test2<'a, T: ?Sized> {
    data: *const T,
    cell: &'a core::cell::Cell<usize>,
}


impl<T: ?Sized> Deref for Test1<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T: ?Sized> Deref for Test2<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

#[no_mangle] fn t1(t: &Test1<i32>) -> i32 {
    **t
}

#[no_mangle] fn t2(t: &Test2<i32>) -> i32 {
    **t
}
