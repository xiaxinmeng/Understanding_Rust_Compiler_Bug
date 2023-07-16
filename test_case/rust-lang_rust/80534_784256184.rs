plain
   Compiling cc v1.0.60
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
error[E0658]: `#[doc(include)]` is experimental
   |
   |
12 | #[doc(include = "core_arch_docs.md")]
   |
   = note: see issue #44732 <https://github.com/rust-lang/rust/issues/44732> for more information
   = note: see issue #44732 <https://github.com/rust-lang/rust/issues/44732> for more information
   = help: add `#![feature(external_doc)]` to the crate attributes to enable
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: aborting due to previous error


For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:23
