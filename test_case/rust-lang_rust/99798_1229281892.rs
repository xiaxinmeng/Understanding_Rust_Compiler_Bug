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
Diff in /checkout/compiler/rustc_infer/src/infer/mod.rs at line 28:
 use rustc_middle::ty::subst::{GenericArg, GenericArgKind, InternalSubsts, SubstsRef};
 use rustc_middle::ty::visit::TypeVisitable;
 pub use rustc_middle::ty::IntVarValue;
-use rustc_middle::ty::{self, GenericParamDefKind, InferConst, Ty, TyCtxt, Subst};
+use rustc_middle::ty::{self, GenericParamDefKind, InferConst, Subst, Ty, TyCtxt};
 use rustc_middle::ty::{ConstVid, FloatVid, IntVid, TyVid};
 use rustc_span::symbol::Symbol;
 use rustc_span::{Span, DUMMY_SP};
Diff in /checkout/compiler/rustc_infer/src/infer/mod.rs at line 1701:
         let tcx = self.tcx;
         if substs.has_infer_types_or_consts() {
             let substs_erased = tcx.erase_regions(unevaluated.substs);
-            let ac = tcx.bound_abstract_const(unevaluated.shrink()).map(|ac| ac.map(|ac| ac.subst(tcx,
-            substs_erased)));
+            let ac = tcx
+                .bound_abstract_const(unevaluated.shrink())
+                .map(|ac| ac.map(|ac| ac.subst(tcx, substs_erased)));
             match ac {
                 Ok(None) => {
                     substs = InternalSubsts::identity_for_item(tcx, unevaluated.def.did);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/common.rs" "/checkout/compiler/rustc_infer/src/infer/combine.rs" "/checkout/compiler/rustc_infer/src/infer/opaque_types/table.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/test_type_match.rs" "/checkout/compiler/rustc_infer/src/infer/mod.rs" "/checkout/compiler/rustc_infer/src/infer/resolve.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/env.rs" "/checkout/compiler/rustc_infer/src/infer/equate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
