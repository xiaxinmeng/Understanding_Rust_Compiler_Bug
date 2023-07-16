plain
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> src/tools/clippy/src/driver.rs:225:19
    |
225 |     let handler = rustc_errors::Handler::with_emitter(true, None, emitter);
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^--------------------- an argument of type `Option<PathBuf>` is missing
note: associated function defined here
   --> /checkout/compiler/rustc_errors/src/lib.rs:593:12
    |
593 |     pub fn with_emitter(
593 |     pub fn with_emitter(
    |            ^^^^^^^^^^^^
help: provide the argument
    |
225 |     let handler = rustc_errors::Handler::with_emitter(true, None, emitter, /* Option<PathBuf> */);

For more information about this error, try `rustc --explain E0061`.
error: could not compile `clippy` due to previous error
warning: build failed, waiting for other jobs to finish...
