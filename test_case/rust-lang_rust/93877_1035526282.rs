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
Diff in /checkout/compiler/rustc_target/src/asm/arm.rs at line 99:
     _target: &Target,
     is_clobber: bool,
 ) -> Result<(), &'static str> {
-    if !is_clobber && target_features.contains(&sym::thumb_mode) && !target_features.contains(&sym::thumb2) {
+    if !is_clobber
+        && target_features.contains(&sym::thumb_mode)
+        && !target_features.contains(&sym::thumb2)
+    {
         Err("high registers (r8+) can only be used as clobbers in Thumb-1 code")
         Ok(())
         Ok(())
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/netbsd_base.rs" "/checkout/compiler/rustc_target/src/asm/arm.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_fuchsia.rs" "/checkout/compiler/rustc_target/src/asm/powerpc.rs" "/checkout/compiler/rustc_target/src/asm/bpf.rs" "/checkout/compiler/rustc_target/src/asm/wasm.rs" "/checkout/compiler/rustc_target/src/asm/msp430.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_linux_gnu_ilp32.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
