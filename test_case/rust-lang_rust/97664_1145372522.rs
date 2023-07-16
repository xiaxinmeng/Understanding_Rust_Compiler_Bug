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
Diff in /checkout/compiler/rustc_typeck/src/coherence/builtin.rs at line 2:
 //! up data structures required by type-checking/codegen.
 
 use crate::errors::{CopyImplOnNonAdt, CopyImplOnTypeWithDtor, DropImplOnWrongItem};
-use std::collections::BTreeMap;
 use rustc_errors::{struct_span_err, MultiSpan};
 use rustc_hir as hir;
 use rustc_hir::def_id::{DefId, LocalDefId};
Diff in /checkout/compiler/rustc_typeck/src/coherence/builtin.rs at line 17:
 use rustc_trait_selection::traits::misc::{can_type_implement_copy, CopyImplementationError};
 use rustc_trait_selection::traits::predicate_for_trait_def;
 use rustc_trait_selection::traits::{self, ObligationCause, TraitEngine, TraitEngineExt};
+use std::collections::BTreeMap;
 
 pub fn check_trait(tcx: TyCtxt<'_>, trait_def_id: DefId) {
     let lang_items = tcx.lang_items();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/coherence/orphan.rs" "/checkout/compiler/rustc_typeck/src/coherence/builtin.rs" "/checkout/compiler/rustc_lint_defs/src/lib.rs" "/checkout/compiler/rustc_lint_defs/src/builtin.rs" "/checkout/compiler/rustc_type_ir/src/lib.rs" "/checkout/compiler/rustc_type_ir/src/codec.rs" "/checkout/compiler/rustc_type_ir/src/sty.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
