plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.45
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0599]: no method named `rfind` found for slice `[u8]` in the current scope
    |
    |
245 |             let index = self.as_bytes()[lower_bound..=index].rfind(
    |                                                              ^^^^^ method not found in `[u8]`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `rfind`, perhaps you need to implement one of them:
            candidate #1: `FuseImpl`
            candidate #2: `DoubleEndedIterator`

error[E0599]: no method named `rfind` found for slice `[u8]` in the current scope
    |
    |
283 |             let index = self.as_bytes()[index..upper_bound].rfind(
    |                                                             ^^^^^ method not found in `[u8]`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `rfind`, perhaps you need to implement one of them:
            candidate #1: `FuseImpl`
            candidate #2: `DoubleEndedIterator`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:04
