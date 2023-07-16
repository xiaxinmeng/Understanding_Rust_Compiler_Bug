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
error: unreachable pattern
   --> library/std/src/io/error.rs:407:13
    |
407 |             Repr::OsWithPath(i, _) => Some(i.into()),
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
   --> library/std/src/io/error.rs:446:13
    |
    |
446 |             Repr::OsWithPath(..) => None,

error: unreachable pattern
   --> library/std/src/io/error.rs:520:13
    |
    |
520 |             Repr::OsWithPath(..) => None,

error: unreachable pattern
   --> library/std/src/io/error.rs:559:13
    |
    |
559 |             Repr::OsWithPath(..) => None,

error: unreachable pattern
   --> library/std/src/io/error.rs:589:13
    |
    |
589 |             Repr::OsWithPath(code, _) => sys::decode_error_kind(code.into()),

error: unreachable pattern
   --> library/std/src/io/error.rs:606:13
    |
    |
606 |             Repr::OsWithPath(code, ref path) => fmt

error: unreachable pattern
   --> library/std/src/io/error.rs:630:13
    |
    |
630 |             Repr::OsWithPath(code, _) => {

error: unreachable pattern
   --> library/std/src/io/error.rs:645:28
    |
    |
645 |             Repr::Os(..) | Repr::OsWithPath(..) | Repr::Simple(..) => self.kind().as_str(),

error: unreachable pattern
   --> library/std/src/io/error.rs:655:13
    |
    |
655 |             Repr::OsWithPath(..) => None,

error: unreachable pattern
   --> library/std/src/io/error.rs:665:13
    |
    |
665 |             Repr::OsWithPath(..) => None,

error: aborting due to 10 previous errors

error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:18
