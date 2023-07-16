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
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error[E0277]: the trait bound `RangeFrom<{integer}>: std::iter::TrustedLen` is not satisfied
    |
    |
188 |     is_trusted_len((0..).take(3));
    |                    ^^^^^^^^^^^^^ the trait `std::iter::TrustedLen` is not implemented for `RangeFrom<{integer}>`
   ::: library/core/tests/iter/mod.rs:27:26
    |
    |
27  | pub fn is_trusted_len<I: TrustedLen>(_: I) {}
    |                          ---------- required by this bound in `iter::is_trusted_len`
    |
    = note: required because of the requirements on the impl of `std::iter::TrustedLen` for `std::iter::Take<RangeFrom<{integer}>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "proc_macro" "-p" "panic_unwind" "-p" "panic_abort" "-p" "term" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:31
