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
    Checking addr2line v0.14.0
error[E0308]: mismatched types
   --> library/std/src/sys/windows/thread_parker.rs:116:86
    |
116 |                 if self.state.compare_exchange(NOTIFIED, EMPTY, Acquire, Acquire) == NOTIFIED {
    |                                                                                      ^^^^^^^^ expected enum `core::result::Result`, found `i8`
    = note: expected enum `core::result::Result<i8, i8>`
               found type `i8`
help: try using a variant of the expected enum
    |
    |
116 |                 if self.state.compare_exchange(NOTIFIED, EMPTY, Acquire, Acquire) == core::prelude::v1::Ok(NOTIFIED) {
    |                                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
116 |                 if self.state.compare_exchange(NOTIFIED, EMPTY, Acquire, Acquire) == Err(NOTIFIED) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:28
