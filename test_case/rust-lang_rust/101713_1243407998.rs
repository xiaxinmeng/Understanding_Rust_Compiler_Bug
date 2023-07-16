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
Diff in /checkout/compiler/rustc_resolve/src/access_levels.rs at line 1:
 use crate::imports::ImportKind;
+use crate::ty::Visibility;
 use crate::NameBinding;
 use crate::NameBindingKind;
 use crate::Resolver;
Diff in /checkout/compiler/rustc_resolve/src/access_levels.rs at line 5:
-use crate::ty::Visibility;
 use rustc_ast::ast;
 use rustc_ast::visit;
 use rustc_ast::visit::Visitor;
Diff in /checkout/compiler/rustc_resolve/src/access_levels.rs at line 12:
 use rustc_hir::def_id::LocalDefId;
 use rustc_hir::def_id::CRATE_DEF_ID;
 use rustc_middle::middle::privacy::AccessLevel;
-use rustc_middle::ty::DefIdTree;
-use rustc_span::sym;
 use rustc_middle::middle::privacy::EffectiveVisibility;
 use rustc_middle::middle::privacy::EffectiveVisibilityValue;
+use rustc_middle::ty::DefIdTree;
+use rustc_span::sym;
 
 pub struct AccessLevelsVisitor<'r, 'a> {
     r: &'r mut Resolver<'a>,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/def_collector.rs" "/checkout/compiler/rustc_resolve/src/access_levels.rs" "/checkout/compiler/rustc_resolve/src/build_reduced_graph.rs" "/checkout/compiler/rustc_attr/src/session_diagnostics.rs" "/checkout/compiler/rustc_attr/src/lib.rs" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/compiler/rustc_const_eval/src/util/alignment.rs" "/checkout/compiler/rustc_resolve/src/diagnostics/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
