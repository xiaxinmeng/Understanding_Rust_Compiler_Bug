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
 }
Diff in /checkout/src/librustdoc/html/render/cache.rs at line 265:
                 get_index_type_name(t, false).map(|name| name.as_str().to_ascii_lowercase())
             .collect::<Vec<_>>();
             .collect::<Vec<_>>();
-        if r.is_empty() {
-        } else {
-            Some(r)
-        }
-        }
+        if r.is_empty() { None } else { Some(r) }
 }
 
 
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
Diff in /checkout/src/librustdoc/html/render/print_item.rs at line 1498:
 
 
 fn document_non_exhaustive_header(item: &clean::Item) -> &str {
-    if item.is_non_exhaustive() {
-        " (Non-exhaustive)"
-        ""
-    }
-    }
+    if item.is_non_exhaustive() { " (Non-exhaustive)" } else { "" }
 
 
 fn document_non_exhaustive(w: &mut Buffer, item: &clean::Item) {
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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/librustdoc/markdown.rs" "/checkout/src/librustdoc/passes/unindent_comments.rs" "/checkout/src/librustdoc/passes/check_code_block_syntax.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/passes/strip_private.rs" "/checkout/src/librustdoc/doctest.rs" "/checkout/src/librustdoc/docfs.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/src/librustdoc/clean/mod.rs at line 237:
                 hir::TraitBoundModifier::None,
         }));
         }));
-        if !v.is_empty() {
-            Some(v)
-            None
-        }
-        }
+        if !v.is_empty() { Some(v) } else { None }
 }
 
