plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.45
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: an `#[unstable]` annotation here has no effect
     |
     |
1382 | #[unstable(feature = "unit_result_default_impl", issue = "none")]
     |
     |
     = note: `#[deny(ineffective_unstable_trait_impl)]` on by default

error: aborting due to previous error

error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:09
