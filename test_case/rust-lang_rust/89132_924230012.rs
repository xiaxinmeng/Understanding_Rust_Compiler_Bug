plain
    Checking rustc-demangle v0.1.21
error[E0308]: mismatched types
    --> library/alloc/src/rc.rs:2794:10
     |
2683 | impl<T: ?Sized, A: Allocator> Weak<T, A> {
     |      - this type parameter
2794 |         (result, alloc)
2794 |         (result, alloc)
     |          ^^^^^^ expected type parameter `T`, found struct `RcBox`
     |
     = note: expected raw pointer `*const T`
                found raw pointer `*mut RcBox<T>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:20
