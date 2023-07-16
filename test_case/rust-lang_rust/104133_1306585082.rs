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
Diff in /checkout/compiler/rustc_traits/src/normalize_erasing_regions.rs at line 2:
 use rustc_middle::traits::query::NoSolution;
 use rustc_middle::ty::{
 use rustc_middle::ty::{
-    self, FallibleTypeFolder, ParamEnvAnd, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable, TypeVisitable,
+    self, FallibleTypeFolder, ParamEnvAnd, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable,
+    TypeVisitable,
 use rustc_trait_selection::traits::query::normalize::AtExt;
 use rustc_trait_selection::traits::query::normalize::AtExt;
 use rustc_trait_selection::traits::{Normalized, ObligationCause};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/chalk/lowering.rs" "/checkout/compiler/rustc_traits/src/chalk/mod.rs" "/checkout/compiler/rustc_traits/src/normalize_erasing_regions.rs" "/checkout/compiler/rustc_traits/src/evaluate_obligation.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_hir_analysis/src/errors.rs" "/checkout/compiler/rustc_hir_analysis/src/structured_errors/sized_unsized_cast.rs" "/checkout/compiler/rustc_traits/src/chalk/db.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
