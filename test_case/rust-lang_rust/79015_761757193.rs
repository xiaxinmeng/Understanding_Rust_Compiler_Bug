plain
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
error[E0599]: no method named `spec_extend_from_within` found for mutable reference `&mut Vec<T, A>` in the current scope
     |
     |
1894 |             self.spec_extend_from_within(range);
     |                  ^^^^^^^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `extend_from_within`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `ExtendFromWithinSpec` defines an item `spec_extend_from_within`, perhaps you need to implement it
     |
     |
2004 | trait ExtendFromWithinSpec {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:35
