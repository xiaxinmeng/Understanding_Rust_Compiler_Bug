 rust
use std::cell::{Ref, RefMut};

pub fn ref_filter_map<
    T: ?Sized,
    U: ?Sized,
    F: FnOnce(&T) -> Option<&U>
>(orig: Ref<T>, f: F) -> Option<Ref<U>> {
    f(&orig)
        .map(|new| new as *const U)
        .map(|raw| Ref::map(orig, |_| unsafe { &*raw }))
}

pub fn ref_mut_filter_map<
    T: ?Sized,
    U: ?Sized,
    F: FnOnce(&mut T) -> Option<&mut U>
>(mut orig: RefMut<T>, f: F) -> Option<RefMut<U>> {
    f(&mut orig)
        .map(|new| new as *mut U)
        .map(|raw| RefMut::map(orig, |_| unsafe { &mut *raw }))
}
