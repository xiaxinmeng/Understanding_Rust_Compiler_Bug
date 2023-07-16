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
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: const-stable function cannot use `#[feature(const_fn)]`
   |
   |
10 |     const fn assert_send_sync<T: Send + Sync>() {}
   |
   |
help: if it is not part of the public API, make this function unstably const
   |
10 |     #[rustc_const_unstable(feature = "...", issue = "...")]
   |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
   |
10 |     #[rustc_allow_const_fn_unstable(const_fn)]


error: const-stable function cannot use `#[feature(const_fn)]`
 --> library/std/src/io/error/tests.rs:8:31
  |
8 |     const fn assert_send_sync<T: Send + Sync>() {}
  |
  |
help: if it is not part of the public API, make this function unstably const
  |
8 |     #[rustc_const_unstable(feature = "...", issue = "...")]
  |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
  |
8 |     #[rustc_allow_const_fn_unstable(const_fn)]

error: aborting due to 2 previous errors

error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "panic_abort" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "term" "-p" "proc_macro" "-p" "std" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:48
