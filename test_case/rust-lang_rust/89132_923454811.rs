plain
    Checking rustc-demangle v0.1.21
error[E0412]: cannot find type `A` in this scope
   --> library/alloc/src/rc.rs:349:53
    |
341 | impl<T: ?Sized> Rc<T> {
    |      - similarly named type parameter `T` defined here
...
349 |     fn from_inner_in(ptr: NonNull<RcBox<T>>, alloc: A) -> Self {
    |
    |
help: there is an enum variant `crate::collections::btree::merge_iter::Peeked::A`; try using the variant's enum
    |
349 |     fn from_inner_in(ptr: NonNull<RcBox<T>>, alloc: crate::collections::btree::merge_iter::Peeked) -> Self {
help: a type parameter with a similar name exists
    |
    |
349 |     fn from_inner_in(ptr: NonNull<RcBox<T>>, alloc: T) -> Self {

error[E0412]: cannot find type `A` in this scope
   --> library/alloc/src/rc.rs:353:54
    |
    |
341 | impl<T: ?Sized> Rc<T> {
    |      - similarly named type parameter `T` defined here
...
353 |     unsafe fn from_ptr_in(ptr: *mut RcBox<T>, alloc: A) -> Self {
    |
    |
help: there is an enum variant `crate::collections::btree::merge_iter::Peeked::A`; try using the variant's enum
    |
353 |     unsafe fn from_ptr_in(ptr: *mut RcBox<T>, alloc: crate::collections::btree::merge_iter::Peeked) -> Self {
help: a type parameter with a similar name exists
    |
    |
353 |     unsafe fn from_ptr_in(ptr: *mut RcBox<T>, alloc: T) -> Self {


error[E0378]: the trait `DispatchFromDyn` may only be implemented for structs containing the field being coerced, ZST fields with 1 byte alignment, and nothing else
    |
    |
329 | impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> DispatchFromDyn<Rc<U, A>> for Rc<T, A> {}
    |
    |
    = note: extra field `alloc` of type `A` is not allowed

error[E0378]: the trait `DispatchFromDyn` may only be implemented for structs containing the field being coerced, ZST fields with 1 byte alignment, and nothing else
     |
     |
2363 | impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> DispatchFromDyn<Weak<U, A>> for Weak<T, A> {}
     |
     |
     = note: extra field `alloc` of type `A` is not allowed
Some errors have detailed explanations: E0378, E0412.
For more information about an error, try `rustc --explain E0378`.
error: could not compile `alloc` due to 4 previous errors
Build completed unsuccessfully in 0:01:24
