plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0605]: non-primitive cast: `u16` as `Box<(dyn core::any::Any + Send + 'static)>`
    |
    |
222 |         panic_any(Box::new(413u16 as Box<dyn Any + Send>));
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
For more information about this error, try `rustc --explain E0605`.
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:02:06
