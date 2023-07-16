plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0599]: the method `into_err` exists for enum `std::result::Result<isize, result::test_into_err::MyNeverToken>`, but its trait bounds were not satisfied
    |
    |
247 |     assert_eq!(until_error_op2().into_err(), 667);
    |                                  ^^^^^^^^ method cannot be called on `std::result::Result<isize, result::test_into_err::MyNeverToken>` due to unsatisfied trait bounds
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `isize: Into<!>`
error[E0308]: mismatched types
   --> library/core/tests/result.rs:244:13
    |
244 |         Err(667)
244 |         Err(667)
    |             ^^^ expected enum `result::test_into_err::MyNeverToken`, found integer
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "proc_macro" "-p" "core" "-p" "panic_unwind" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:44
