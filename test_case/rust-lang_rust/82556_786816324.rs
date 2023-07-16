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
Diff in /checkout/compiler/rustc_target/src/spec/s390x_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::s390x_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/powerpc64_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::powerpc64_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/powerpc64le_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::powerpc64le_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/mips_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::mips_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/aarch64_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::aarch64_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/armv7_unknown_linux_dynmusleabihf.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::armv7_unknown_linux_musleabihf::target();
Diff in /checkout/compiler/rustc_target/src/spec/i586_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::i586_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/mips64_unknown_linux_dynmuslabi64.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::mips64_unknown_linux_muslabi64::target();
Diff in /checkout/compiler/rustc_target/src/spec/mipsel_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::mipsel_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/i686_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::i686_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/powerpc_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::powerpc_unknown_linux_musl::target();
Diff in /checkout/compiler/rustc_target/src/spec/x86_64_unknown_linux_dynmusl.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::x86_64_unknown_linux_musl::target();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/powerpc64_unknown_linux_musl.rs" "/checkout/compiler/rustc_target/src/spec/s390x_unknown_linux_dynmusl.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_fuchsia.rs" "/checkout/compiler/rustc_target/src/spec/armv7a_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/s390x_unknown_linux_musl.rs" "/checkout/compiler/rustc_target/src/abi/call/mips64.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_openbsd.rs" "/checkout/compiler/rustc_target/src/abi/call/nvptx.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
Build completed unsuccessfully in 0:00:14
Diff in /checkout/compiler/rustc_target/src/spec/arm_unknown_linux_dynmusleabihf.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::arm_unknown_linux_musleabihf::target();
Diff in /checkout/compiler/rustc_target/src/spec/mips64el_unknown_linux_dynmuslabi64.rs at line 1:
-use crate::spec::Target;
 use crate::spec::crt_objects::new;
+use crate::spec::Target;
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::mips64el_unknown_linux_muslabi64::target();
