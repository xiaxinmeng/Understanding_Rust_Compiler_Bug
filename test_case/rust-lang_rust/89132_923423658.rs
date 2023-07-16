plain
    Checking rustc-demangle v0.1.21
error[E0425]: cannot find value `layout` in this scope
   --> library/alloc/src/rc.rs:658:42
    |
658 |             Err(_) => handle_alloc_error(layout),


error[E0107]: this struct takes 1 generic argument but 2 generic arguments were supplied
     |
     |
2875 | impl<T: ?Sized, A: Allocator> RcInnerPtr for RcBox<T, A> {
     |                                              ^^^^^    - help: remove this generic argument
     |                                              expected 1 generic argument
     |
     |
note: struct defined here, with 1 generic parameter: `T`
     |
     |
290  | struct RcBox<T: ?Sized> {


error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
     |
     |
2875 | impl<T: ?Sized, A: Allocator> RcInnerPtr for RcBox<T, A> {

Some errors have detailed explanations: E0107, E0207, E0425.
For more information about an error, try `rustc --explain E0107`.
error: could not compile `alloc` due to 3 previous errors
