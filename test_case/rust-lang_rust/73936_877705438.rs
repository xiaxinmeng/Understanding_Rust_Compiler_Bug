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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/html/layout.rs at line 67:
     let style_files = style_files
         .iter()
         .filter_map(|t| {
-            if let Some(stem) = t.path.file_stem() {
-                Some((stem, t.disabled))
-                None
-            }
-            }
+            if let Some(stem) = t.path.file_stem() { Some((stem, t.disabled)) } else { None }
         })
         .filter_map(|t| if let Some(path) = t.0.to_str() { Some((path, t.1)) } else { None })
         .map(|t| {
Diff in /checkout/src/librustdoc/html/render/context.rs at line 141:
     /// Returns the `collapsed_doc_value` of the given item if this is the main crate, otherwise
     /// returns the `doc_value`.
     crate fn maybe_collapsed_doc_value<'a>(&self, item: &'a clean::Item) -> Option<String> {
-        if self.collapsed {
-            item.collapsed_doc_value()
-            item.doc_value()
-        }
-        }
+        if self.collapsed { item.collapsed_doc_value() } else { item.doc_value() }
 
 
     crate fn edition(&self) -> Edition {
Diff in /checkout/src/librustdoc/clean/auto_trait.rs at line 600:
                                             });
                                             });
                                             continue; // If something other than a Fn ends up
-                                                      // with parenthesis, leave it alone
+                                            // with parenthesis, leave it alone
                                     }
 
Diff in /checkout/src/librustdoc/clean/types.rs at line 620:
Diff in /checkout/src/librustdoc/clean/types.rs at line 620:
                 classes.push("deprecated");
 
 
-            if !classes.is_empty() {
-                Some(classes.join(" "))
-                None
-            }
-            }
+            if !classes.is_empty() { Some(classes.join(" ")) } else { None }
     }
 
Diff in /checkout/src/librustdoc/clean/types.rs at line 863:
             }
             }
         }
 
-        if cfg == Cfg::True {
-        } else {
-        } else {
-            Some(Arc::new(cfg))
-        }
+        if cfg == Cfg::True { None } else { Some(Arc::new(cfg)) }
 }
 
Diff in /checkout/src/librustdoc/clean/types.rs at line 1086:
             }
             }
             add_doc_fragment(&mut out, &new_frag);
         }
-        if out.is_empty() {
-        } else {
-            Some(out)
-        }
-        }
+        if out.is_empty() { None } else { Some(out) }
 
 
     /// Return the doc-comments on this item, grouped by the module they came from.
Diff in /checkout/src/librustdoc/clean/types.rs at line 1109:
     /// Finds all `doc` attributes as NameValues and returns their corresponding values, joined
     /// with newlines.
     crate fn collapsed_doc_value(&self) -> Option<String> {
-        if self.doc_strings.is_empty() {
-        } else {
-        } else {
-            Some(self.doc_strings.iter().collect())
-        }
+        if self.doc_strings.is_empty() { None } else { Some(self.doc_strings.iter().collect()) }
 
 
     crate fn get_doc_aliases(&self) -> Box<[String]> {
Diff in /checkout/src/librustdoc/doctest.rs at line 621:
         s.lines()
             .map(|line| {
                 let comment = line.find("//");
-                if let Some(comment_begins) = comment {
-                    &line[0..comment_begins]
-                    line
-                }
-                }
+                if let Some(comment_begins) = comment { &line[0..comment_begins] } else { line }
             })
             .any(|code| code.contains("fn main"))
Diff in /checkout/src/librustdoc/doctest.rs at line 1039:
Diff in /checkout/src/librustdoc/doctest.rs at line 1039:
         if let Some(ref source_map) = self.source_map {
             let line = self.position.lo().to_usize();
             let line = source_map.lookup_char_pos(BytePos(line as u32)).line;
-            if line > 0 {
-            } else {
-                line
-            }
-            }
+            if line > 0 { line - 1 } else { line }
             0
         }
         }
Diff in /checkout/src/librustdoc/clean/simplify.rs at line 140:
         .iter()
         .filter_map(|(pred, _)| {
             if let ty::PredicateKind::Trait(pred, _) = pred.kind().skip_binder() {
-                if pred.trait_ref.self_ty() == self_ty {
-                    Some(pred.def_id())
-                    None
-                }
-                }
+                if pred.trait_ref.self_ty() == self_ty { Some(pred.def_id()) } else { None }
                 None
             }
             }
Diff in /checkout/src/librustdoc/visit_ast.rs at line 25:
     let relative = tcx.def_path(did).data.into_iter().filter_map(|elem| {
         // extern blocks have an empty name
         let s = elem.data.to_string();
-        if !s.is_empty() {
-            Some(s)
-            None
-        }
-        }
+        if !s.is_empty() { Some(s) } else { None }
     });
     std::iter::once(crate_name).chain(relative).collect()
Diff in /checkout/src/librustdoc/passes/collect_intra_doc_links.rs at line 806:
                     _ => false,
                 };
 
 
-            if saw_impl {
-                Some(trait_)
-                None
-            }
-            }
+            if saw_impl { Some(trait_) } else { None }
     });
     iter.collect()
Diff in /checkout/src/librustdoc/clean/inline.rs at line 168:
Diff in /checkout/src/librustdoc/clean/inline.rs at line 168:
     let relative = cx.tcx.def_path(did).data.into_iter().filter_map(|elem| {
         // extern blocks have an empty name
         let s = elem.data.to_string();
-        if !s.is_empty() {
-            Some(s)
-            None
-        }
-        }
+        if !s.is_empty() { Some(s) } else { None }
     });
     let fqn = if let ItemType::Macro = kind {
         // Check to see if it is a macro 2.0 or built-in macro
Diff in /checkout/src/librustdoc/html/format.rs at line 340:
         let expr = self.expr(tcx);
             move |f| {
             move |f| {
-                if f.alternate() {
-                    f.write_str(&expr)
-                } else {
-                    write!(f, "{}", Escape(&expr))
-                }
+                if f.alternate() { f.write_str(&expr) } else { write!(f, "{}", Escape(&expr)) }
         )
     }
     }
Diff in /checkout/src/librustdoc/html/format.rs at line 480:
     let cache = &cx.cache();
     let relative_to = &cx.current;
     fn to_module_fqp(shortty: ItemType, fqp: &[String]) -> &[String] {
-        if shortty == ItemType::Module {
-            &fqp[..]
-        } else {
-            &fqp[..fqp.len() - 1]
-        }
+        if shortty == ItemType::Module { &fqp[..] } else { &fqp[..fqp.len() - 1] }
 
 
     if !did.is_local() && !cache.access_levels.is_public(did) && !cache.document_private {
Diff in /checkout/src/librustdoc/html/format.rs at line 1158:
         let arrow = if let hir::IsAsync::Async = asyncness {
             let output = self.sugared_async_return_type();
             arrow_plain = format!("{:#}", output.print(cx));
-            if f.alternate() {
-                arrow_plain.clone()
-            } else {
-                format!("{}", output.print(cx))
-            }
+            if f.alternate() { arrow_plain.clone() } else { format!("{}", output.print(cx)) }
         } else {
             arrow_plain = format!("{:#}", self.output.print(cx));
-            if f.alternate() {
-                arrow_plain.clone()
-            } else {
-                format!("{}", self.output.print(cx))
-            }
+            if f.alternate() { arrow_plain.clone() } else { format!("{}", self.output.print(cx)) }
 
 
         let declaration_len = header_len + args_plain.len() + arrow_plain.len();
Diff in /checkout/src/librustdoc/html/format.rs at line 1413:
 
 
 crate fn print_default_space<'a>(v: bool) -> &'a str {
-    if v {
-        "default "
-        ""
-    }
-    }
+    if v { "default " } else { "" }
 
 
 impl clean::GenericArg {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/unindent_comments.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/passes/check_code_block_syntax.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/passes/strip_private.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/passes/propagate_doc_cfg.rs" "/checkout/src/librustdoc/visit_ast.rs"` failed.
Build completed unsuccessfully in 0:00:15
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
