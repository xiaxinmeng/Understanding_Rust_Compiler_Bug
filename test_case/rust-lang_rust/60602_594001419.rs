
   Compiling backtrace v0.3.44
error[E0658]: use of unstable library feature 'ptr_cast'
   --> C:\Users\jack_\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.44\src\backtrace\dbghelp.rs:110:74
    |
110 |                 RtlLookupFunctionEntry(addr, &mut base, ptr::null_mut()).cast()
    |                                                                          ^^^^
    |
    = note: for more information, see https://github.com/rust-lang/rust/issues/60602

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: Could not compile `backtrace`.
warning: build failed, waiting for other jobs to finish...
error: failed to compile `cargo-generate v0.5.0`, intermediate artifacts can be found at `C:\Users\jack_\AppData\Local\Temp\cargo-installM3JL39`

Caused by:
  build failed
