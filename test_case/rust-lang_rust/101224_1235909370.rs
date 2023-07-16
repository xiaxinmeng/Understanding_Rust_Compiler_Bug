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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_ast_lowering/src/lib.rs at line 1335:
                     }
                     ImplTraitContext::InTrait => {
                         self.lower_impl_trait_in_trait(span, def_node_id, |lctx| {
-                            lctx.lower_param_bounds(
-                                bounds,
-                                ImplTraitContext::InTrait,
-                            )
+                            lctx.lower_param_bounds(bounds, ImplTraitContext::InTrait)
                     }
                     }
                     ImplTraitContext::Universal => {
Diff in /checkout/compiler/rustc_typeck/src/check/mod.rs at line 132:
 use crate::util::common::indenter;
 
 use self::coercion::DynamicCoerceMany;
-use self::region::region_scope_tree;
 use self::compare_method::compare_predicates_and_trait_impl_trait_tys;
+use self::region::region_scope_tree;
 pub use self::Expectation::*;
 #[macro_export]
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 1602:
             }
             }
             ItemKind::ImplTraitPlaceholder(_) => {
                 let parent_id = tcx.hir().get_parent_item(hir_id).to_def_id();
-                assert!(matches!(tcx.def_kind(parent_id), DefKind::AssocFn | DefKind::ImplTraitPlaceholder));
+                assert!(matches!(
+                    tcx.def_kind(parent_id),
+                    DefKind::AssocFn | DefKind::ImplTraitPlaceholder
                 Some(parent_id)
             }
             _ => None,
             _ => None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/compiler/rustc_ast_lowering/src/path.rs" "/checkout/compiler/rustc_ast_lowering/src/pat.rs" "/checkout/compiler/rustc_ast_lowering/src/lib.rs" "/checkout/compiler/rustc_ast_lowering/src/errors.rs" "/checkout/compiler/rustc_ast_lowering/src/block.rs" "/checkout/compiler/rustc_ast_lowering/src/index.rs" "/checkout/compiler/rustc_attr/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
