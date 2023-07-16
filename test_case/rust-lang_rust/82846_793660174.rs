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
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 400:
     ) -> bool {
         let tcx = self.tcx;
         let err_fn = move |span: Span, msg: &str| {
-            tcx.sess
-                .span_err(
-                    span,
-                    &format!(
-                        "`#[doc(alias{})]` {}",
-                        if is_list { "(\"...\")" } else { " = \"...\"" },
-                        msg,
-                );
-                );
+            tcx.sess.span_err(
+                span,
+                &format!(
+                    "`#[doc(alias{})]` {}",
+                    if is_list { "(\"...\")" } else { " = \"...\"" },
+                    msg,
+            );
             false
         };
         };
         if doc_alias.is_empty() {
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 420:
         if let Some(c) =
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/hygiene.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_passes/src/diagnostic_items.rs" "/checkout/compiler/rustc_passes/src/upvars.rs" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_span/src/analyze_source_file.rs" "/checkout/compiler/rustc_passes/src/dead.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs"` failed.
             doc_alias.chars().find(|&c| c == '"' || c == '\'' || (c.is_whitespace() && c != ' '))
-            self.tcx
-                .sess
-                .span_err(
-                .span_err(
-                    meta.name_value_literal_span().unwrap_or_else(|| meta.span()),
-                    &format!(
-                        "{:?} character isn't allowed in `#[doc(alias{})]`",
-                        c,
-                        if is_list { "(\"...\")" } else { " = \"...\"" },
-                );
-                );
+            self.tcx.sess.span_err(
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                meta.name_value_literal_span().unwrap_or_else(|| meta.span()),
+                &format!(
+                    "{:?} character isn't allowed in `#[doc(alias{})]`",
+                    c,
+                    if is_list { "(\"...\")" } else { " = \"...\"" },
+            );
             return false;
         }
         }
         if doc_alias.starts_with(' ') || doc_alias.ends_with(' ') {
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 466:
         }
         let item_name = self.tcx.hir().name(hir_id);
         if &*item_name.as_str() == doc_alias {
-            return err_fn( meta.span(), "is the same as the item's name");
+            return err_fn(meta.span(), "is the same as the item's name");
         true
     }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
