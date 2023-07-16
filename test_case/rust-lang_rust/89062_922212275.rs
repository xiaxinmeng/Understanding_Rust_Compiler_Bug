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
Diff in /checkout/compiler/rustc_target/src/spec/x86_64_unknown_none_elf.rs at line 22:
     Target {
         llvm_target: "x86_64-unknown-none-elf".to_string(),
         pointer_width: 64,
-        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".to_string(),
+        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
+            .to_string(),
         arch: "x86_64".to_string(),
         options: opts,
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/mipsisa32r6el_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/riscv32i_unknown_none_elf.rs" "/checkout/compiler/rustc_target/src/spec/sparc64_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_freebsd.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_none_elf.rs" "/checkout/compiler/rustc_target/src/spec/android_base.rs" "/checkout/compiler/rustc_target/src/spec/arm_unknown_linux_musleabihf.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_fuchsia.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
