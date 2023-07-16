plain
   Compiling rustc-demangle v0.1.21
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


error[E0375]: implementing the trait `CoerceUnsized` requires multiple coercions
     |
     |
2360 | impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Weak<U>> for Weak<T, A> {}
     |                                                      ^^^^^^^^^^^^^^^^^^^^^^ requires multiple coercions
     |
     = note: `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced
     = note: currently, 2 fields need coercions: `ptr` (`NonNull<RcBox<T>>` to `NonNull<RcBox<U>>`), `alloc` (`A` to `Global`)

error[E0378]: implementing the `DispatchFromDyn` trait requires multiple coercions
     |
     |
2363 | impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> DispatchFromDyn<Weak<U>> for Weak<T, A> {}
     |
     |
     = note: the trait `DispatchFromDyn` may only be implemented for a coercion between structures with a single field being coerced
     = note: currently, 2 fields need coercions: `ptr` (`NonNull<RcBox<T>>` to `NonNull<RcBox<U>>`), `alloc` (`A` to `Global`)
Some errors have detailed explanations: E0375, E0378, E0412.
For more information about an error, try `rustc --explain E0375`.
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
