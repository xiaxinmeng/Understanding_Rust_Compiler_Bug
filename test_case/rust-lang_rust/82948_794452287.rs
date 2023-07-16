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
    Checking std v0.0.0 (/checkout/library/std)
error: expected identifier, found reserved identifier `_`
 --> library/std/src/thread/tests.rs:9:8
  |
9 | static _: () = {
  |        ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
 --> library/std/src/backtrace/tests.rs:3:8
  |
  |
3 | static _: () = {
  |        ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
 --> library/std/src/io/error/tests.rs:7:8
  |
  |
7 | static _: () = {
  |        ^ expected identifier, found reserved identifier
error: aborting due to 3 previous errors

error: could not compile `std`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "core" "-p" "std" "-p" "term" "-p" "proc_macro" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:43
