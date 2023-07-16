plain
    Checking rustc_driver_impl v0.0.0 (/checkout/compiler/rustc_driver_impl)
error[E0521]: borrowed data escapes outside of closure
    --> compiler/rustc_driver_impl/src/lib.rs:1253:17
     |
1247 |     panic::set_hook(Box::new(move |info| {
     |                                    ---- `info` is a reference that is only valid in the closure body
...
1253 |                 early_error_no_abort(ErrorOutputType::default(), msg.as_str());
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `info` escapes the closure body here
error[E0521]: borrowed data escapes outside of closure
    --> compiler/rustc_driver_impl/src/lib.rs:1253:17
     |
     |
1247 |     panic::set_hook(Box::new(move |info| {
     |                                    |
     |                                    |
     |                                    `info` is a reference that is only valid in the closure body
     |                                    has type `&std::panic::PanicInfo<'2>`
...
1253 |                 early_error_no_abort(ErrorOutputType::default(), msg.as_str());
     |                 |
     |                 |
     |                 `info` escapes the closure body here
     |                 argument requires that `'2` must outlive `'static`
For more information about this error, try `rustc --explain E0521`.
error: could not compile `rustc_driver_impl` (lib test) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_driver_impl` (lib) due to 2 previous errors
