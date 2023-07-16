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
Diff in /checkout/compiler/rustc_middle/src/ty/vtable.rs at line 1:
 use std::convert::TryFrom;
-use rustc_ast::Mutability;
 use crate::mir::interpret::{alloc_range, AllocId, Allocation, Pointer, Scalar};
 use crate::mir::interpret::{alloc_range, AllocId, Allocation, Pointer, Scalar};
-use crate::ty::{self, DefId, SubstsRef, Ty, TyCtxt};
 use crate::ty::fold::TypeFoldable;
+use crate::ty::{self, DefId, SubstsRef, Ty, TyCtxt};
+use rustc_ast::Mutability;
 
 #[derive(Clone, Copy, Debug, PartialEq, HashStable)]
 pub enum VtblEntry<'tcx> {
Diff in /checkout/compiler/rustc_middle/src/ty/vtable.rs at line 37:
         drop(vtables_cache);
 
         // See https://github.com/rust-lang/rust/pull/86475#discussion_r655162674
-        assert!(!ty.needs_subst() && !poly_trait_ref.map_or(false, |trait_ref| trait_ref.needs_subst()));
+        assert!(
+            !ty.needs_subst() && !poly_trait_ref.map_or(false, |trait_ref| trait_ref.needs_subst())
+        );
         let param_env = ty::ParamEnv::empty();
         let vtable_entries = if let Some(poly_trait_ref) = poly_trait_ref {
             let trait_ref = poly_trait_ref.with_self_ty(tcx, ty);
Diff in /checkout/compiler/rustc_middle/src/ty/context.rs at line 11:
 use crate::middle::cstore::{CrateStoreDyn, EncodedMetadata};
 use crate::middle::resolve_lifetime::{self, LifetimeScopeForPath, ObjectLifetimeDefault};
 use crate::middle::stability;
-use crate::mir::interpret::{self, Allocation, AllocId, ConstValue, Scalar};
+use crate::mir::interpret::{self, AllocId, Allocation, ConstValue, Scalar};
 use crate::mir::{Body, Field, Local, Place, PlaceElem, ProjectionKind, Promoted};
 use crate::thir::Thir;
 use crate::traits;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/interpret/pointer.rs" "/checkout/compiler/rustc_expand/src/module.rs" "/checkout/compiler/rustc_middle/src/util/common/tests.rs" "/checkout/compiler/rustc_middle/src/ty/context.rs" "/checkout/compiler/rustc_middle/src/ty/fast_reject.rs" "/checkout/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs" "/checkout/compiler/rustc_middle/src/util/bug.rs" "/checkout/compiler/rustc_middle/src/ich/impls_hir.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
