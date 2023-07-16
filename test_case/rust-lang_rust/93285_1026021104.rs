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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/project.rs at line 22:
 use rustc_data_structures::sso::SsoHashSet;
 use rustc_data_structures::stack::ensure_sufficient_stack;
-use rustc_hir::def_id::DefId;
 use rustc_hir::def::DefKind;
+use rustc_hir::def_id::DefId;
 use rustc_hir::lang_items::LangItem;
 use rustc_hir::lang_items::LangItem;
 use rustc_infer::infer::resolve::OpportunisticRegionResolver;
 use rustc_middle::ty::fold::{TypeFoldable, TypeFolder};
Diff in /checkout/compiler/rustc_trait_selection/src/traits/project.rs at line 1565:
     impl_source: Selection<'tcx>,
 ) -> Progress<'tcx> {
     match impl_source {
-        super::ImplSource::UserDefined(data) => {
-            confirm_impl_candidate(selcx, obligation, data)
-        }
+        super::ImplSource::UserDefined(data) => confirm_impl_candidate(selcx, obligation, data),
         super::ImplSource::Generator(data) => confirm_generator_candidate(selcx, obligation, data),
         super::ImplSource::Closure(data) => confirm_closure_candidate(selcx, obligation, data),
         super::ImplSource::FnPointer(data) => confirm_fn_pointer_candidate(selcx, obligation, data),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/project.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/on_unimplemented.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
