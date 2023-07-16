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
Diff in /checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs at line 5:
     let mut base = super::freebsd_base::opts();
     base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m32".to_string());
     // Extra hint to linker that we are generating secure-PLT code.
-    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("--target=powerpc-unknown-freebsd13.0".to_string());
+    base.pre_link_args
+        .entry(LinkerFlavor::Gcc)
+        .or_default()
+        .push("--target=powerpc-unknown-freebsd13.0".to_string());
     base.max_atomic_width = Some(32);
     Target {
     Target {
Diff in /checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs at line 14:
         data_layout: "E-m:e-p:32:32-i64:64-n32".to_string(),
         arch: "powerpc".to_string(),
         options: TargetOptions {
-             endian: Endian::Big,
-             features: "+secure-plt".to_string(),
-             relocation_model: RelocModel::Pic,
-             mcount: "_mcount".to_string(),
-             ..base
+            endian: Endian::Big,
+            features: "+secure-plt".to_string(),
+            relocation_model: RelocModel::Pic,
+            mcount: "_mcount".to_string(),
+            ..base
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/armv7r_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/armv5te_unknown_linux_uclibceabi.rs" "/checkout/compiler/rustc_target/src/spec/i686_uwp_windows_gnu.rs" "/checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_hermit.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_linux_android.rs" "/checkout/compiler/rustc_target/src/spec/abi.rs" "/checkout/compiler/rustc_target/src/spec/riscv32imac_unknown_none_elf.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
