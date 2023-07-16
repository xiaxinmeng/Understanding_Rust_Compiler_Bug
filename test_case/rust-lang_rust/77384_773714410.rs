plain
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0264]: unknown external lang item: `backtrace_create`
   |
   |
98 |     fn backtrace_create(ip: usize) -> *mut dyn RawBacktrace;


error[E0264]: unknown external lang item: `backtrace_enabled`
    |
101 |     fn backtrace_enabled() -> bool;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0264]: unknown external lang item: `backtrace_status`
    |
    |
104 |     fn backtrace_status(raw: *mut dyn RawBacktrace) -> BacktraceStatus;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0264`.
For more information about this error, try `rustc --explain E0264`.
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:16
