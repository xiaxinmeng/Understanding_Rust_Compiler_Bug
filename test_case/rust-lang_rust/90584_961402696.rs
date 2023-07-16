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
Diff in /checkout/compiler/rustc_target/src/spec/armv7_sony_vita_eabihf.rs at line 1:
-use crate::spec::{LinkArgs, LinkerFlavor, PanicStrategy, RelocModel, Target, TargetOptions};
 use crate::abi::Endian;
+use crate::spec::{LinkArgs, LinkerFlavor, PanicStrategy, RelocModel, Target, TargetOptions};
 
 /// A base target for PlayStation Vita devices using the VITASDK toolchain.
 ///
Diff in /checkout/compiler/rustc_target/src/spec/armv7_sony_vita_eabihf.rs at line 7:
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut pre_link_args = LinkArgs::new();
-    pre_link_args.insert(
-        LinkerFlavor::Gcc,
-        vec![
-            "-Wl,-q".to_string(),
-    );
-    );
+    pre_link_args.insert(LinkerFlavor::Gcc, vec!["-Wl,-q".to_string()]);
     Target {
     Target {
         llvm_target: "armv7a-vita-eabihf".to_string(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_none.rs" "/checkout/compiler/rustc_target/src/spec/armebv7r_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/i486_unknown_linux_gnu.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_target/src/spec/armv7_sony_vita_eabihf.rs" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_target/src/spec/i686_linux_android.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/caller_location.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
