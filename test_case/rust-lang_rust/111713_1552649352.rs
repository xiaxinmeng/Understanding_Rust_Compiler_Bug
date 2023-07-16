plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0425]: cannot find value `threads` in this scope
   --> compiler/rustc_interface/src/util.rs:170:41
    |
170 |     run_in_thread_with_globals(edition, threads, f)
    |                                         ^^^^^^^ help: a local variable with a similar name exists: `_threads`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_interface` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_interface` (lib test) due to previous error
