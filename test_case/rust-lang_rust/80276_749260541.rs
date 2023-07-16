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
    Checking object v0.22.0
    Checking hashbrown v0.9.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error: use of associated function `core::sync::atomic::AtomicI8::compare_and_swap` that will be deprecated in future version 1.50.0: Use `compare_exchange` or `compare_exchange_weak` instead
   --> library/std/src/sys/windows/thread_parker.rs:116:31
    |
116 |                 if self.state.compare_and_swap(NOTIFIED, EMPTY, Acquire) == NOTIFIED {
    |
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:34
