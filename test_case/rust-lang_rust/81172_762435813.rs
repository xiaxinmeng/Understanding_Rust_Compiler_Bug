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
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: unused import: `self`
 --> library/core/tests/ptr.rs:2:17
  |
2 | use core::ptr::{self, *};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `Debug`, `Display`
 --> library/core/tests/ptr.rs:3:16
  |
3 | use std::fmt::{Debug, Display};
  |                ^^^^^  ^^^^^^^
error: aborting due to 2 previous errors

error: could not compile `core`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_abort" "-p" "panic_unwind" "-p" "alloc" "-p" "unwind" "-p" "term" "-p" "std" "-p" "proc_macro" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:48
