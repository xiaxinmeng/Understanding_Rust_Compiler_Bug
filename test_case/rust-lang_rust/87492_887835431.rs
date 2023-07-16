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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/check_unsafety.rs at line 611:
 
     // Closures are handled by their owner, if it has a body, except if the
     // closure occurs in an AnonConst body to avoid a cycle (issue #87414)
-    if tcx.is_closure(def.did.to_def_id())
-    {
+    if tcx.is_closure(def.did.to_def_id()) {
         let encl_body_owner = tcx.hir().enclosing_body_owner(hir_id);
         if matches!(tcx.hir().get(encl_body_owner), hir::Node::AnonConst(_)) {
             tcx.ensure().thir_check_unsafety(tcx.hir().local_def_id(encl_body_owner));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_passes/src/node_count.rs" "/checkout/compiler/rustc_ast_passes/src/feature_gate.rs" "/checkout/compiler/rustc_ast_passes/src/ast_validation.rs" "/checkout/compiler/rustc_traits/src/type_op.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/compiler/rustc_traits/src/normalize_erasing_regions.rs" "/checkout/compiler/rustc_mir_build/src/check_unsafety.rs" "/checkout/compiler/rustc_ast_passes/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
