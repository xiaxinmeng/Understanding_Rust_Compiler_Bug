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
Diff in /checkout/compiler/rustc_typeck/src/check_unused.rs at line 21:
     unused_crates_lint(tcx);
 
 
-impl <'v, 'tcx> ItemLikeVisitor<'v> for CheckVisitor<'tcx> {
+impl<'v, 'tcx> ItemLikeVisitor<'v> for CheckVisitor<'tcx> {
     fn visit_item(&mut self, item: &hir::Item<'_>) {
         if item.vis.node.is_pub() || item.span.is_dummy() {
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 223:
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 223:
     err.emit();
 
 
-fn reject_placeholder_type_signatures_in_item<'tcx>(tcx: TyCtxt<'tcx>, item: &'tcx hir::Item<'tcx>) {
+fn reject_placeholder_type_signatures_in_item<'tcx>(
+    tcx: TyCtxt<'tcx>,
+    item: &'tcx hir::Item<'tcx>,
+) {
     let (generics, suggest) = match &item.kind {
         hir::ItemKind::Union(_, generics)
         | hir::ItemKind::Enum(_, generics)
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 1488:
 }
 
 
-fn e0307<'fcx, 'tcx> (fcx: &FnCtxt<'fcx, 'tcx>, span: Span, receiver_ty: Ty<'_>) {
+fn e0307<'fcx, 'tcx>(fcx: &FnCtxt<'fcx, 'tcx>, span: Span, receiver_ty: Ty<'_>) {
     struct_span_err!(
         fcx.tcx.sess.diagnostic(),
         span,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/mbe/transcribe.rs" "/checkout/compiler/rustc_lint_defs/src/builtin.rs" "/checkout/compiler/rustc_lint_defs/src/lib.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs" "/checkout/compiler/rustc_typeck/src/coherence/orphan.rs" "/checkout/compiler/rustc_typeck/src/check_unused.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_parser.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
