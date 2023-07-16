plain
    Checking rustc-demangle v0.1.21
error: associated function is never used: `allocate_for_ptr`
    --> library/alloc/src/rc.rs:1681:15
     |
1681 |     unsafe fn allocate_for_ptr(ptr: *const T) -> *mut RcBox<T> {
     |
     |
     = note: `-D dead-code` implied by `-D warnings`
error: associated function is never used: `from_box`
    --> library/alloc/src/rc.rs:1693:8
     |
     |
1693 |     fn from_box(v: Box<T>) -> Rc<T> {

error: associated function is never used: `copy_from_slice_in`
    --> library/alloc/src/rc.rs:1846:15
     |
     |
1846 |     unsafe fn copy_from_slice_in(v: &[T], alloc: A) -> Rc<[T], A> {

error: associated function is never used: `from_iter_exact_in`
    --> library/alloc/src/rc.rs:1858:15
     |
