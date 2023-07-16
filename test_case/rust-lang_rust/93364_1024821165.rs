plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0782]: trait objects must include the `dyn` keyword
   |
   |
97 | impl<S> Join<crate::string::ToString> for [S] 
   |
   |
help: add `dyn` keyword before this trait
   |
97 | impl<S> Join<dyn crate::string::ToString> for [S] 


error[E0277]: the size for values of type `(dyn ToString + 'static)` cannot be known at compilation time
    |
    |
97  | impl<S> Join<crate::string::ToString> for [S] 
    |
    |
    = help: the trait `Sized` is not implemented for `(dyn ToString + 'static)`
note: required by a bound in `Join`
    |
    |
749 | pub trait Join<Separator> {
    |                ^^^^^^^^^ required by this bound in `Join`
help: consider relaxing the implicit `Sized` restriction
    |
749 | pub trait Join<Separator: ?Sized> {

Some errors have detailed explanations: E0277, E0782.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `alloc` due to 2 previous errors
