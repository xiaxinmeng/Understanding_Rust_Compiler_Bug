plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.79
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `Option<bool>: ~const cmp::PartialEq<_>` is not satisfied
   |
   |
39 |         (self as *const u8).guaranteed_eq(null()) == Some(true)
   |                                                   ^^ no implementation for `Option<bool> == _`
   |
   = help: the trait `~const cmp::PartialEq<_>` is not implemented for `Option<bool>`
note: the trait `cmp::PartialEq<_>` is implemented for `Option<bool>`, but that implementation is not `const`
   |
   |
39 |         (self as *const u8).guaranteed_eq(null()) == Some(true)


error[E0277]: the trait bound `Option<bool>: ~const cmp::PartialEq<_>` is not satisfied
   |
   |
38 |         (self as *mut u8).guaranteed_eq(null_mut()) == Some(true)
   |                                                     ^^ no implementation for `Option<bool> == _`
   |
   = help: the trait `~const cmp::PartialEq<_>` is not implemented for `Option<bool>`
note: the trait `cmp::PartialEq<_>` is implemented for `Option<bool>`, but that implementation is not `const`
   |
   |
38 |         (self as *mut u8).guaranteed_eq(null_mut()) == Some(true)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:23
