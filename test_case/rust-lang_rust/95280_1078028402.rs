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
Diff in /checkout/compiler/rustc_traits/src/dropck_outlives.rs at line 9:
 use rustc_span::source_map::{Span, DUMMY_SP};
 use rustc_trait_selection::traits::query::dropck_outlives::trivial_dropck_outlives;
 use rustc_trait_selection::traits::query::dropck_outlives::{
-    DropckOutlivesResult, DropckConstraint,
+    DropckConstraint, DropckOutlivesResult,
 use rustc_trait_selection::traits::query::normalize::AtExt;
 use rustc_trait_selection::traits::query::normalize::AtExt;
 use rustc_trait_selection::traits::query::{CanonicalTyGoal, NoSolution};
Diff in /checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs at line 5:
 use rustc_middle::ty::subst::GenericArg;
 use rustc_middle::ty::{self, Ty, TyCtxt};
 
-pub use rustc_middle::traits::query::{DropckOutlivesResult, DropckConstraint};
+pub use rustc_middle::traits::query::{DropckConstraint, DropckOutlivesResult};
 
 pub trait AtExt<'tcx> {
     fn dropck_outlives(&self, ty: Ty<'tcx>) -> InferOk<'tcx, Vec<GenericArg<'tcx>>>;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/diagnostics.rs" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs" "/checkout/compiler/rustc_resolve/src/access_levels.rs" "/checkout/compiler/rustc_traits/src/dropck_outlives.rs" "/checkout/compiler/rustc_traits/src/type_op.rs" "/checkout/compiler/rustc_traits/src/normalize_projection_ty.rs" "/checkout/compiler/rustc_traits/src/lib.rs" "/checkout/compiler/rustc_resolve/src/build_reduced_graph.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
