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
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
   |
   |
11 |     assert_send_sync::<JoinHandle<()>>();


error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
   |
   |
12 |     assert_send_sync::<Thread>();


error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
 --> library/std/src/backtrace/tests.rs:5:5
  |
5 |     assert_send_sync::<Backtrace>();


error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
 --> library/std/src/io/error/tests.rs:9:5
  |
9 |     assert_send_sync::<Error>();

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0015`.
For more information about this error, try `rustc --explain E0015`.
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_abort" "-p" "core" "-p" "term" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "proc_macro" "-p" "panic_unwind" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:43
