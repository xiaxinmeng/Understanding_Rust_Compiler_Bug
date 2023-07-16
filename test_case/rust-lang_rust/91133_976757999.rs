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
Diff in /checkout/compiler/rustc_parse/src/parser/item.rs at line 939:
             Ok(items) => {
                 let module = ast::ForeignMod { unsafety, abi, items };
                 Ok((Ident::empty(), ItemKind::ForeignMod(module)))
+            }
             Err(mut err) => {
             Err(mut err) => {
-                    let current_qual_sp = self.prev_token.span;
-                    let current_qual_sp = current_qual_sp.to(sp_start);
-                    if let Ok(current_qual) = self.span_to_snippet(current_qual_sp) {
-                        if err.message() == "expected `{`, found keyword `unsafe`" {
-                            let invalid_qual_sp = self.token.uninterpolated_span();
-                            let invalid_qual = self.span_to_snippet(invalid_qual_sp).unwrap();
+                let current_qual_sp = self.prev_token.span;
+                let current_qual_sp = current_qual_sp.to(sp_start);
+                if let Ok(current_qual) = self.span_to_snippet(current_qual_sp) {
+                    if err.message() == "expected `{`, found keyword `unsafe`" {
+                        let invalid_qual_sp = self.token.uninterpolated_span();
+                        let invalid_qual = self.span_to_snippet(invalid_qual_sp).unwrap();
-                            err.span_suggestion(
+                        err.span_suggestion(
+                        err.span_suggestion(
                                 current_qual_sp.to(invalid_qual_sp),
                                 &format!("`{}` must come before `{}`", invalid_qual, current_qual),
                                 format!("{} {}", invalid_qual, current_qual),
Diff in /checkout/compiler/rustc_parse/src/parser/item.rs at line 955:
                                 Applicability::MachineApplicable,
                             ).note("keyword order for functions declaration is `default`, `pub`, `const`, `async`, `unsafe`, `extern`");
                     }
+                }
                 Err(err)
-            },
-            },
+            }
         }
     }
 
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs" "/checkout/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs" "/checkout/compiler/rustc_parse/src/parser/pat.rs" "/checkout/compiler/rustc_mir_transform/src/add_moves_for_packed_drops.rs" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_mir_transform/src/simplify.rs" "/checkout/compiler/rustc_parse/src/parser/mod.rs" "/checkout/compiler/rustc_mir_transform/src/add_retag.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
