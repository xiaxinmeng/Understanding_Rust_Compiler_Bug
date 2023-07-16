plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0119]: conflicting implementations of trait `slice::Join<&_>` for type `[&_]`
    |
    |
97  | / impl<S> Join<S> for [S] 
98  | | where S: crate::string::ToString {
99  | |     type Output = String;
...   |
108 | |     }
109 | | }
    | |_^ conflicting implementation for `[&_]`
    | |_^ conflicting implementation for `[&_]`
    |
   ::: library/alloc/src/slice.rs:776:1
    |
776 |   impl<T: Clone, V: Borrow<[T]>> Join<&T> for [V] {
    |   ----------------------------------------------- first implementation here
    |
    = note: downstream crates may implement trait `core::borrow::Borrow<[_]>` for type `&_`
For more information about this error, try `rustc --explain E0119`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:22
