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
Diff in /checkout/compiler/rustc_target/src/spec/mipsel_sony_psp.rs at line 6:
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut pre_link_args = LinkArgs::new();
-    pre_link_args.insert(
-        LinkerFlavor::Lld(LldFlavor::Ld),
-        vec!["--emit-relocs".into(), "--nmagic".into()],
+    pre_link_args
+    pre_link_args
+        .insert(LinkerFlavor::Lld(LldFlavor::Ld), vec!["--emit-relocs".into(), "--nmagic".into()]);
     Target {
     Target {
         llvm_target: "mipsel-sony-psp".into(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/armebv7r_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/mips64_unknown_linux_gnuabi64.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_openbsd.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/avr_gnu_base.rs" "/checkout/compiler/rustc_target/src/spec/mipsel_sony_psp.rs" "/checkout/compiler/rustc_target/src/spec/linux_base.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
