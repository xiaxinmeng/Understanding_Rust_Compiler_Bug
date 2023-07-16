plain
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0599]: no method named `assert_len` found for type parameter `R` in the current scope
     |
     |
1986 |         let range = src.assert_len(self.len());
     |                         ^^^^^^^^^^ method not found in `R`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `alloc`
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:45
