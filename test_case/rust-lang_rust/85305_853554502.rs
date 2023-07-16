plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.18
error[E0658]: pattern bindings after an `@` are unstable
    |
    |
874 |         let src @ Range { start, end } = slice::range(src, ..self.len());
    |
    = note: see issue #65490 <https://github.com/rust-lang/rust/issues/65490> for more information
    = help: add `#![feature(bindings_after_at)]` to the crate attributes to enable


error[E0658]: pattern bindings after an `@` are unstable
    |
    |
874 |         let src @ Range { start, end } = slice::range(src, ..self.len());
    |
    = note: see issue #65490 <https://github.com/rust-lang/rust/issues/65490> for more information
    = help: add `#![feature(bindings_after_at)]` to the crate attributes to enable


error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `alloc`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:10
