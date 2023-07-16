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
Diff in /checkout/compiler/rustc_target/src/spec/powerpc64le_unknown_freebsd.rs at line 7:
     base.max_atomic_width = Some(64);
     Target {
-       llvm_target: "powerpc64le-unknown-freebsd".to_string(),
-       pointer_width: 64,
-       pointer_width: 64,
-       data_layout: "e-m:e-i64:64-n32:64".to_string(),
-       arch: "powerpc64".to_string(),
-       options: TargetOptions { mcount: "_mcount".to_string(), ..base },
+        llvm_target: "powerpc64le-unknown-freebsd".to_string(),
+        pointer_width: 64,
+        data_layout: "e-m:e-i64:64-n32:64".to_string(),
+        arch: "powerpc64".to_string(),
+        options: TargetOptions { mcount: "_mcount".to_string(), ..base },
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/armv7r_none_eabi.rs" "/checkout/compiler/rustc_target/src/spec/sparc64_unknown_openbsd.rs" "/checkout/compiler/rustc_target/src/asm/spirv.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7em_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_uwp_windows_msvc.rs" "/checkout/compiler/rustc_target/src/asm/wasm.rs" "/checkout/compiler/rustc_target/src/spec/powerpc64le_unknown_freebsd.rs" "/checkout/compiler/rustc_target/src/asm/x86.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
