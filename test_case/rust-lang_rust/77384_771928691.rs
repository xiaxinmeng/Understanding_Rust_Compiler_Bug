plain
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
    |
    |
254 | / impl Backtrace {
255 | |     /// Returns whether backtrace captures are enabled through environment
256 | |     /// variables.
257 | |     fn enabled() -> bool {
368 | |     }
369 | | }
369 | | }
    | |_^ impl for type defined outside of crate.
    |
    = note: define and implement a trait or new type instead

error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
    |
    |
371 | / impl<'a> Backtrace {
372 | |     /// Returns an iterator over the backtrace frames.
373 | |     #[unstable(feature = "backtrace_frames", issue = "79676")]
374 | |     pub fn frames(&'a self) -> &'a [BacktraceFrame] {
375 | |         if let Inner::Captured(c) = &self.inner { &c.force().frames } else { &[] }
377 | | }
377 | | }
    | |_^ impl for type defined outside of crate.
    |
    = note: define and implement a trait or new type instead
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0116`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:23
