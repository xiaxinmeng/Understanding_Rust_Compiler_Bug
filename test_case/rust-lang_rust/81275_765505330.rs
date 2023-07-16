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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/formats/renderer.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/src/librustdoc/formats/renderer.rs at line 70:
     });
     });
     let (mut format_renderer, mut krate) = {
-        tcx.sess.prof.generic_activity_with_arg("create_renderer", T::descr()).run(|| {
-            T::init(krate, options, render_info, edition, &mut cache, tcx)
-        })?
+        tcx.sess
+            .prof
+            .generic_activity_with_arg("create_renderer", T::descr())
+            .run(|| T::init(krate, options, render_info, edition, &mut cache, tcx))?
 
 
     let cache = Arc::new(cache);
Diff in /checkout/src/librustdoc/formats/renderer.rs at line 113:
 
             cx.mod_item_out(&name)?;
         } else if item.name.is_some() {
-            tcx.sess.prof.generic_activity_with_arg("render_item", &*item.name.unwrap_or(unknown).as_str()).run(|| {
-                cx.item(item, &cache)
-            })?;
+            tcx.sess
+                .prof
+                .generic_activity_with_arg("render_item", &*item.name.unwrap_or(unknown).as_str())
+                .run(|| cx.item(item, &cache))?;
     }
 
 
Diff in /checkout/src/librustdoc/formats/renderer.rs at line 122:
-    tcx.sess.prof.generic_activity_with_arg("renderer_after_krate", T::descr()).run(|| {
-        format_renderer.after_krate(&krate, &cache, diag)
-    })
+    tcx.sess
+        .prof
+        .generic_activity_with_arg("renderer_after_krate", T::descr())
+        .run(|| format_renderer.after_krate(&krate, &cache, diag))
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:17
