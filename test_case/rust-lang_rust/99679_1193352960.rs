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
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/spec/aarch64_unknown_none.rs at line 6:
 //
 // For example, `-C target-cpu=cortex-a53`.
 
-use super::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, Target, TargetOptions, SanitizerSet};
+use super::{
+    LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, SanitizerSet, Target, TargetOptions,
 
 pub fn target() -> Target {
     let opts = TargetOptions {
Diff in /checkout/compiler/rustc_target/src/spec/riscv64imac_unknown_none_elf.rs at line 1:
Diff in /checkout/compiler/rustc_target/src/spec/riscv64imac_unknown_none_elf.rs at line 1:
-use crate::spec::{CodeModel, Target, TargetOptions, SanitizerSet};
+use crate::spec::{CodeModel, SanitizerSet, Target, TargetOptions};
 use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel};
 pub fn target() -> Target {
 pub fn target() -> Target {
Diff in /checkout/compiler/rustc_target/src/spec/x86_64_unknown_none.rs at line 5:
 // features.
 use super::{
 use super::{
-    CodeModel, LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, RelroLevel, StackProbeType,
-    Target, TargetOptions, SanitizerSet,
+    CodeModel, LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, RelroLevel, SanitizerSet,
+    StackProbeType, Target, TargetOptions,
 
 pub fn target() -> Target {
 pub fn target() -> Target {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_none.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_tvos.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_uefi.rs" "/checkout/compiler/rustc_typeck/src/check/expectation.rs" "/checkout/compiler/rustc_target/src/spec/sparcv9_sun_solaris.rs" "/checkout/compiler/rustc_typeck/src/check/mod.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsicck.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
