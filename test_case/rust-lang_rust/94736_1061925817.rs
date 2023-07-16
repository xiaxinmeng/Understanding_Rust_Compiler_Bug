plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs at line 10:
     Target {
         pointer_width: 64,
         pointer_width: 64,
-        data_layout: "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".to_string(),
+        data_layout: "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
+            .to_string(),
         arch: "x86_64".to_string(),
         options: TargetOptions {
             max_atomic_width: Some(64),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/i686_unknown_haiku.rs" "/checkout/compiler/rustc_target/src/spec/l4re_base.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_watchos_sim.rs" "/checkout/compiler/rustc_target/src/spec/thumbv6m_none_eabi.rs" "/checkout/compiler/rustc_target/src/spec/i686_apple_darwin.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_uwp_windows_msvc.rs" "/checkout/compiler/rustc_target/src/spec/riscv64gc_unknown_none_elf.rs" "/checkout/compiler/rustc_target/src/spec/mips_unknown_linux_musl.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
