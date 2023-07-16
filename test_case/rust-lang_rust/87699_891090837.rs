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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/spec/apple_base.rs at line 93:
 
 pub fn ios_llvm_target(arch: &str) -> String {
     let (major, minor) = ios_deployment_target();
-    format!("{}-apple-ios{}.{}.0", arch , major, minor)
+    format!("{}-apple-ios{}.{}.0", arch, major, minor)
 
 
 pub fn ios_sim_llvm_target(arch: &str) -> String {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/abi.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7em_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/riscv32gc_unknown_linux_musl.rs" "/checkout/compiler/rustc_target/src/spec/vxworks_base.rs" "/checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs" "/checkout/compiler/rustc_target/src/spec/linux_base.rs" "/checkout/compiler/rustc_target/src/spec/apple_base.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
