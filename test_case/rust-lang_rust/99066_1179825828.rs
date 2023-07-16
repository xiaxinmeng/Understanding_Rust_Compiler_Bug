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
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/librustdoc/clean/inline.rs at line 124:
 
     let (attrs, cfg) = merge_attrs(cx, Some(parent_module), load_attrs(cx, did), attrs_clone);
     cx.inlined.insert(did.into());
-    let mut item =
-        clean::Item::from_def_id_and_attrs_and_parts(did, Some(name), kind, Box::new(attrs), cx, cfg);
+    let mut item = clean::Item::from_def_id_and_attrs_and_parts(
+        did,
+        Some(name),
+        Box::new(attrs),
+        cx,
+        cfg,
+    );
+    );
     if let Some(import_def_id) = import_def_id {
         // The visibility needs to reflect the one from the reexport and not from the "source" DefId.
         item.visibility = cx.tcx.visibility(import_def_id).clean(cx);
Diff in /checkout/src/librustdoc/html/format.rs at line 743:
         if f != r {
             let dissimilar_part_count = relative_to_fqp.len() - i;
             let fqp_module = &fqp[i..fqp.len()];
-            return Box::new(iter::repeat(sym::dotdot)
-                .take(dissimilar_part_count)
-                .chain(fqp_module.iter().copied()));
+            return Box::new(
+                iter::repeat(sym::dotdot)
+                    .take(dissimilar_part_count)
+                    .chain(fqp_module.iter().copied()),
         }
     }
     }
     // e.g. linking to std::sync::atomic from std::sync
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/highlight/fixtures/sample.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/bootstrap/job.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/bootstrap/dylib_util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', format.rs:175:19
Build completed unsuccessfully in 0:00:15
