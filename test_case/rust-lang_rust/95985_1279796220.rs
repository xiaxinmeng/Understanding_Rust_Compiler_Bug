plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0277]: `*mut ()` cannot be shared between threads safely
   |
   |
24 |     static CONTEXT: Context<'static> = Context::from_waker(&WAKER);
   |                     ^^^^^^^^^^^^^^^^ `*mut ()` cannot be shared between threads safely
   |
   = help: within `Context<'static>`, the trait `Sync` is not implemented for `*mut ()`
   = note: required because it appears within the type `PhantomData<*mut ()>`
   = note: required because it appears within the type `Context<'static>`
   = note: shared static variables must have a type that implements `Sync`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:55
