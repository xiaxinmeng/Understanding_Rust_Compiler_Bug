plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
414 |     pub const fn into_inner(self) -> T {
    |                             ^^^^ constant functions cannot evaluate destructors
415 |         self.value.into_inner()
    |     - value is dropped here


error[E0493]: destructors cannot be evaluated at compile-time
    |
    |
703 |     pub const fn into_inner(self) -> T {
    |                             ^^^^ constant functions cannot evaluate destructors
707 |     }
    |     - value is dropped here


error[E0493]: destructors cannot be evaluated at compile-time
     |
     |
1837 |     pub const fn into_inner(self) -> T {
     |                             ^^^^ constant functions cannot evaluate destructors
1839 |     }
     |     - value is dropped here

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0493`.
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:30
