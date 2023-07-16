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
Diff in /checkout/compiler/rustc_passes/src/stability.rs at line 2:
 //! propagating default levels lexically from parent to children ast nodes.
 use rustc_attr::{
 use rustc_attr::{
-    self as attr, ConstStability, Stability, StabilityLevel, Unstable, UnstableReason,
-    rust_version_symbol, VERSION_PLACEHOLDER,
+    self as attr, rust_version_symbol, ConstStability, Stability, StabilityLevel, Unstable,
+    UnstableReason, VERSION_PLACEHOLDER,
 };
 use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexMap};
 use rustc_errors::{struct_span_err, Applicability};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/lib.rs" "/checkout/compiler/rustc_passes/src/reachable.rs" "/checkout/compiler/rustc_codegen_llvm/src/builder.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_codegen_llvm/src/value.rs" "/checkout/compiler/rustc_passes/src/lib.rs" "/checkout/compiler/rustc_passes/src/liveness.rs" "/checkout/compiler/rustc_passes/src/entry.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
