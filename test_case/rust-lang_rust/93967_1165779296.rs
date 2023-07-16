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
#################                                                         24.1%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/hir/map/mod.rs at line 281:
                     })) if anon_const.hir_id == hir_id => true,
                 };
-                if inline {
-                    DefKind::InlineConst
-                } else {
-                } else {
-                    DefKind::AnonConst
-                }
+                if inline { DefKind::InlineConst } else { DefKind::AnonConst }
             }
             Node::Field(_) => DefKind::Field,
             Node::Expr(expr) => match expr.kind {
Diff in /checkout/compiler/rustc_middle/src/hir/map/mod.rs at line 1077:
 
 
     pub fn span_if_local(self, id: DefId) -> Option<Span> {
-        if id.is_local() {
-            Some(self.tcx.def_span(id))
-            None
-        }
-        }
+        if id.is_local() { Some(self.tcx.def_span(id)) } else { None }
 
 
     pub fn res_span(self, res: Res) -> Option<Span> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/subst.rs" "/checkout/compiler/rustc_middle/src/dep_graph/dep_node.rs" "/checkout/compiler/rustc_middle/src/infer/canonical.rs" "/checkout/compiler/rustc_middle/src/infer/unify_key.rs" "/checkout/compiler/rustc_middle/src/infer/mod.rs" "/checkout/compiler/rustc_middle/src/hir/nested_filter.rs" "/checkout/compiler/rustc_middle/src/hir/map/mod.rs" "/checkout/compiler/rustc_middle/src/ty/context.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
