plain
    Checking rustc-demangle v0.1.21
error[E0425]: cannot find value `alloc` in this scope
    --> library/alloc/src/rc.rs:1703:34
     |
1703 |             box_free(box_unique, alloc);
     |                                  ^^^^^ a field by this name exists in `Self`
help: consider importing this function
     |
246  | use crate::alloc::alloc;
     |
---
    |
note: associated function defined here
   --> library/alloc/src/rc.rs:729:12
    |
729 |     pub fn try_new_in(value: T, alloc: A) -> Result<Self, AllocError> {

error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1719:23
     |
     |
1710 | impl<T: ?Sized, A: Allocator> Rc<T, A> {
     |      - this type parameter
...
1719 |                 |mem| (ptr as *mut RcBox<T>).set_ptr_value(mem),
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found type parameter `T`
     |
     = note: expected raw pointer `*mut RcBox<()>`
                found raw pointer `*mut RcBox<T>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1716:13
     |
     |
1710 |   impl<T: ?Sized, A: Allocator> Rc<T, A> {
     |        - this type parameter
...
1713 |       unsafe fn allocate_for_ptr_in(ptr: *const T, alloc: &A) -> *mut RcBox<T> {
     |                                                                  ------------- expected `*mut RcBox<T>` because of return type
1716 | /             Rc::<()>::allocate_for_layout(
1717 | |                 Layout::for_value(&*ptr),
1717 | |                 Layout::for_value(&*ptr),
1718 | |                 |layout| alloc.allocate(layout),
1719 | |                 |mem| (ptr as *mut RcBox<T>).set_ptr_value(mem),
     | |_____________^ expected type parameter `T`, found `()`
     |
     |
     = note: expected raw pointer `*mut RcBox<T>`
                found raw pointer `*mut RcBox<()>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1831:23
     |
     |
1831 |                 |mem| ptr::slice_from_raw_parts_mut(mem as *mut T, len) as *mut RcBox<[T]>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found slice
     |
     = note: expected raw pointer `*mut RcBox<()>`
                found raw pointer `*mut RcBox<[T]>`
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:1828:13
     |
     |
1826 |       unsafe fn allocate_for_slice_in(len: usize, alloc: &A) -> *mut RcBox<[T]> {
     |                                                                 --------------- expected `*mut RcBox<[T]>` because of return type
1828 | /             Rc::<()>::allocate_for_layout(
1828 | /             Rc::<()>::allocate_for_layout(
1829 | |                 Layout::array::<T>(len).unwrap(),
1830 | |                 |layout| alloc.allocate(layout),
1831 | |                 |mem| ptr::slice_from_raw_parts_mut(mem as *mut T, len) as *mut RcBox<[T]>,
     | |_____________^ expected slice, found `()`
     |
     |
     = note: expected raw pointer `*mut RcBox<[T]>`
                found raw pointer `*mut RcBox<()>`
Some errors have detailed explanations: E0061, E0308, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `alloc` due to 6 previous errors
Build completed unsuccessfully in 0:01:17
