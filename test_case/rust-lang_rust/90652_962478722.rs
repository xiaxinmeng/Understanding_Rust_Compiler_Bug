plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0378]: implementing the `DispatchFromDyn` trait requires multiple coercions
   --> library/core/src/ptr/unique.rs:163:1
    |
163 | impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Unique<U>> for Unique<T> where T: Unsize<U> {}
    |
    |
    = note: the trait `DispatchFromDyn` may only be implemented for a coercion between structures with a single field being coerced
    = note: currently, 2 fields need coercions: `pointer` (`*const T` to `*const U`), `_marker` (`PhantomData<T>` to `PhantomData<U>`)
For more information about this error, try `rustc --explain E0378`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
