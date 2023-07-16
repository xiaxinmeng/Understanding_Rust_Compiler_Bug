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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/asm.rs at line 656:
         InlineAsmRegClass::AArch64(AArch64InlineAsmRegClass::reg) => modifier,
         InlineAsmRegClass::AArch64(AArch64InlineAsmRegClass::vreg)
         | InlineAsmRegClass::AArch64(AArch64InlineAsmRegClass::vreg_low16) => {
-            if modifier == Some('v') {
-            } else {
-                modifier
-            }
-            }
+            if modifier == Some('v') { None } else { modifier }
         }
         InlineAsmRegClass::AArch64(AArch64InlineAsmRegClass::preg) => {
             unreachable!("clobber-only")
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/asm.rs" "/checkout/library/std/src/thread/mod.rs" "/checkout/library/std/src/panic/tests.rs" "/checkout/library/std/src/backtrace.rs" "/checkout/library/std/src/panicking.rs" "/checkout/library/std/src/io/impls.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/write.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
