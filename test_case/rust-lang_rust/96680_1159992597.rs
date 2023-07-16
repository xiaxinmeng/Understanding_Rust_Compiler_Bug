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
Diff in /checkout/src/librustdoc/clean/types.rs at line 642:
                 classes.push("deprecated");
             }
 
-            if !classes.is_empty() {
-                Some(classes.join(" "))
-                None
-            }
-            }
+            if !classes.is_empty() { Some(classes.join(" ")) } else { None }
     }
 
Diff in /checkout/src/librustdoc/clean/types.rs at line 950:
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
 
Diff in /checkout/src/librustdoc/clean/types.rs at line 1233:
Diff in /checkout/src/librustdoc/clean/types.rs at line 1233:
             add_doc_fragment(&mut out, new_frag);
         }
         out.pop();
-        if out.is_empty() {
-        } else {
-            Some(out)
-        }
-        }
+        if out.is_empty() { None } else { Some(out) }
 
 
     /// Return the doc-comments on this item, grouped by the module they came from.
Diff in /checkout/src/librustdoc/clean/types.rs at line 2289:
 impl Term {
 impl Term {
     pub(crate) fn ty(&self) -> Option<&Type> {
-        if let Term::Type(ty) = self {
-            Some(ty)
-            None
-        }
-        }
+        if let Term::Type(ty) = self { Some(ty) } else { None }
 }
 
Diff in /checkout/src/librustdoc/clean/types.rs at line 2508:
 
 
 impl SubstParam {
     pub(crate) fn as_ty(&self) -> Option<&Type> {
-        if let Self::Type(ty) = self {
-            Some(ty)
-            None
-        }
-        }
+        if let Self::Type(ty) = self { Some(ty) } else { None }
 
 
     pub(crate) fn as_lt(&self) -> Option<&Lifetime> {
Diff in /checkout/src/librustdoc/clean/types.rs at line 2519:
-        if let Self::Lifetime(lt) = self {
-            Some(lt)
-            None
-        }
-        }
+        if let Self::Lifetime(lt) = self { Some(lt) } else { None }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/tools/unicode-table-generator/src/raw_emitter.rs" "/checkout/src/tools/unicode-table-generator/src/main.rs" "/checkout/src/tools/unicode-table-generator/src/range_search.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/tools/lint-docs/src/main.rs" "/checkout/src/tools/tidy/src/lib.rs" "/checkout/src/tools/unicode-table-generator/src/skiplist.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
