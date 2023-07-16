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
Diff in /checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs at line 3:
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::freebsd_base::opts();
+    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m32".to_string());
+    // Extra hint to linker that we are generating secure-PLT code.
     base.pre_link_args
         .entry(LinkerFlavor::Gcc)
         .or_default()
Diff in /checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs at line 9:
-        .push("-m32".to_string());
-    // Extra hint to linker that we are generating secure-PLT code.
-    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("--target=powerpc-unknown-freebsd13.0".to_string());
+        .push("--target=powerpc-unknown-freebsd13.0".to_string());
     base.max_atomic_width = Some(32);
     Target {
     Target {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/mips64el_unknown_linux_gnuabi64.rs" "/checkout/compiler/rustc_target/src/spec/powerpc_unknown_freebsd.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7a_uwp_windows_msvc.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_ios_macabi.rs" "/checkout/compiler/rustc_target/src/spec/sparc64_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_none_hermitkernel.rs" "/checkout/compiler/rustc_target/src/spec/i686_unknown_uefi.rs" "/checkout/compiler/rustc_target/src/spec/thumbv8m_base_none_eabi.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
