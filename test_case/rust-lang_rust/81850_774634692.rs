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
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error: variant is never constructed: `Read`
 --> library/std/src/sys_common/rwlock.rs:4:5
4 |     Read,
  |     ^^^^
  |
  |
  = note: `-D dead-code` implied by `-D warnings`

error: variant is never constructed: `Write`
 --> library/std/src/sys_common/rwlock.rs:5:5
5 |     Write,
  |     ^^^^^


error: associated function is never used: `read_with_guard`
  --> library/std/src/sys_common/rwlock.rs:55:19
   |
55 |     pub unsafe fn read_with_guard(&'static self) -> RWLockGuard {


error: associated function is never used: `write_with_guard`
  --> library/std/src/sys_common/rwlock.rs:90:19
   |
90 |     pub unsafe fn write_with_guard(&'static self) -> RWLockGuard {

error: aborting due to 4 previous errors

error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:27
