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
  |
2 | #![feature(command_access)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-attributes` implied by `-D warnings`
error: crate-level attribute should be in the root module
 --> library/std/src/sys/windows/process/tests.rs:2:1
  |
2 | #![feature(command_access)]
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "proc_macro" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "term" "-p" "std" "-p" "panic_abort" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:29
