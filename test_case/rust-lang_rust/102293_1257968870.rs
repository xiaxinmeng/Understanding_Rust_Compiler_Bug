plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/spec/powerpc64_ibm_aix.rs at line 3:
 pub fn target() -> Target {
     let mut base = super::aix_base::opts();
     base.max_atomic_width = Some(64);
-    base.add_pre_link_args(LinkerFlavor::Ld,
+    base.add_pre_link_args(
+        LinkerFlavor::Ld,
         &[
             "-b64".into(),
             "-bpT:0x100000000".into(),
Diff in /checkout/compiler/rustc_target/src/spec/powerpc64_ibm_aix.rs at line 10:
             "-bpD:0x110000000".into(),
             "-bcdtors:all:0:s".into(),
+        ],
+    );
 
     Target {
     Target {
         llvm_target: "powerpc64-ibm-aix".into(),
Diff in /checkout/compiler/rustc_target/src/spec/aix_base.rs at line 1:
 use crate::abi::Endian;
-use crate::spec::{cvs, crt_objects, LinkArgs, LinkerFlavor, LinkOutputKind,
-                  CodeModel, TargetOptions};
+use crate::spec::{
+    crt_objects, cvs, CodeModel, LinkArgs, LinkOutputKind, LinkerFlavor, TargetOptions,
 
 pub fn opts() -> TargetOptions {
 pub fn opts() -> TargetOptions {
     let mut late_link_args = LinkArgs::new();
Diff in /checkout/compiler/rustc_target/src/spec/aix_base.rs at line 7:
-    late_link_args.insert(LinkerFlavor::Ld,
-                          vec![
-                              "-lunwind".into(),
-                              "-lpthreads".into(),
-                              "-lm".into(),
-                              "-lc".into(),
+    late_link_args.insert(
+    late_link_args.insert(
+        LinkerFlavor::Ld,
+        vec!["-lunwind".into(), "-lpthreads".into(), "-lm".into(), "-lc".into()],
 
     TargetOptions {
     TargetOptions {
         abi: "vec-extabi".into(),
Diff in /checkout/compiler/rustc_target/src/spec/aix_base.rs at line 34:
         default_dwarf_version: 3,
         function_sections: false,
         pre_link_objects: crt_objects::new(&[
-            (LinkOutputKind::DynamicNoPicExe,
-             &["/usr/lib/crt0_64.o", "/usr/lib/crti_64.o"]),
-            (LinkOutputKind::DynamicPicExe,
-             &["/usr/lib/crt0_64.o", "/usr/lib/crti_64.o"]),
+            (LinkOutputKind::DynamicNoPicExe, &["/usr/lib/crt0_64.o", "/usr/lib/crti_64.o"]),
+            (LinkOutputKind::DynamicPicExe, &["/usr/lib/crt0_64.o", "/usr/lib/crti_64.o"]),
         ]),
         late_link_args,
         dll_suffix: ".a".into(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/armv7r_none_eabi.rs" "/checkout/compiler/rustc_target/src/spec/s390x_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/linux_base.rs" "/checkout/compiler/rustc_target/src/spec/solaris_base.rs" "/checkout/compiler/rustc_target/src/spec/arm_unknown_linux_musleabihf.rs" "/checkout/compiler/rustc_target/src/spec/powerpc64_ibm_aix.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_apple_ios_macabi.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_kmc_solid_asp3.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
