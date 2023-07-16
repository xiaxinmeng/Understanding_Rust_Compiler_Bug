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
Diff in /checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs at line 741:
             items: methods.into_iter().chain(associated_types).collect(),
         }));
         SpanMarker { container_id: cx.current_expansion.id, span: self.span }
-        .visit_item_kind(&mut item);
-        cx.item(
-            self.span,
-            Ident::empty(),
-            a,
-        )
-        )
+            .visit_item_kind(&mut item);
+        cx.item(self.span, Ident::empty(), a, item)
 
     fn expand_struct_def(
     fn expand_struct_def(
Diff in /checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs at line 1803:
     span: Span,
 
 
-    // let mut marker = Marker(cx.current_expansion.id, transparency);
+// let mut marker = Marker(cx.current_expansion.id, transparency);
 impl MutVisitor for SpanMarker {
     fn visit_span(&mut self, sp: &mut Span) {
         // let x = sp.ctxt();
Diff in /checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs at line 1813:
         let y = sp.ctxt().outer_expn();
         tracing::info!(?self.span, ?sp, ?self.container_id, ?x, ?y);
         // if self.span != *sp {
-            // *sp = sp.apply_mark(self.container_id.to_expn_id(), rustc_span::hygiene::Transparency::SemiTransparent);
+        // *sp = sp.apply_mark(self.container_id.to_expn_id(), rustc_span::hygiene::Transparency::SemiTransparent);
         //     // *sp = sp.with_ctxt(self.span.ctxt());
         // } else {
-            // *sp = sp.with_ctxt(self.span.ctxt());
+        // *sp = sp.with_ctxt(self.span.ctxt());
         // }
-        if self.container_id.to_expn_id() != sp.ctxt().outer_expn() { // && sp.ctxt().outer_expn() == rustc_span::hygiene::ExpnId::root() {
+        if self.container_id.to_expn_id() != sp.ctxt().outer_expn() {
+            // && sp.ctxt().outer_expn() == rustc_span::hygiene::ExpnId::root() {
             // println!("{:?} {:?} {:?}", self.ctxt, sp.ctxt(), sp);
             // tracing::info!("marked");
             // *sp.with_ctxt();
Diff in /checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs at line 1825:
-            *sp = sp.apply_mark(self.container_id.to_expn_id(), rustc_span::hygiene::Transparency::SemiTransparent);
+            *sp = sp.apply_mark(
+                self.container_id.to_expn_id(),
+                rustc_span::hygiene::Transparency::SemiTransparent,
+            );
             // *sp = sp.apply_mark(self.container_id.to_expn_id(), rustc_span::hygiene::Transparency::Transparent);
             // sp.apply_mark(self.ctxt.outer_expn(), rustc_span::hygiene::Transparency::Transparent);
             // *sp = sp.apply_mark(self.container_id.to_expn_id(), rustc_span::hygiene::Transparency::Opaque);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/deriving/default.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/bounds.rs" "/checkout/compiler/rustc_builtin_macros/src/format.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs" "/checkout/compiler/rustc_builtin_macros/src/format_foreign/shell/tests.rs" "/checkout/compiler/rustc_builtin_macros/src/standard_library_imports.rs" "/checkout/compiler/rustc_builtin_macros/src/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
