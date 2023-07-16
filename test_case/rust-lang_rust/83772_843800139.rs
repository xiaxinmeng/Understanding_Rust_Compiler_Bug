plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.43
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
803 |         R: Try<Ok = B>;
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
811 |         R: Try<Ok = B>;
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
838 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
883 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
930 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found

error[E0220]: associated type `Ok` not found for `try::Try`
    |
    |
975 |         R: Try<Ok = B>,
    |                ^^ associated type `Ok` not found
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0220`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:04
