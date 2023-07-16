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
Diff in /checkout/compiler/rustc_metadata/src/creader.rs at line 599:
         // already loaded.
         let root = library.metadata.get_root();
         // Proc-macro crates are always compiled for the host target, so they should be reused even if we're cross-compiling.
-        Ok(Some(if locator.triple == self.sess.opts.target_triple || locator.is_proc_macro == Some(true) {
-            let mut result = LoadResult::Loaded(library);
-            self.cstore.iter_crate_data(|cnum, data| {
-                if data.name() == root.name() && root.hash() == data.hash() {
-                    assert!(locator.hash.is_none());
-                    info!("load success, going to previous cnum: {}", cnum);
-                    result = LoadResult::Previous(cnum);
-            });
-            result
-        } else {
-            LoadResult::Loaded(library)
-            LoadResult::Loaded(library)
-        }))
+        Ok(Some(
+            if locator.triple == self.sess.opts.target_triple || locator.is_proc_macro == Some(true)
+            {
+                let mut result = LoadResult::Loaded(library);
+                self.cstore.iter_crate_data(|cnum, data| {
+                    if data.name() == root.name() && root.hash() == data.hash() {
+                        assert!(locator.hash.is_none());
+                        info!("load success, going to previous cnum: {}", cnum);
+                        result = LoadResult::Previous(cnum);
+                });
+                result
+            } else {
+                LoadResult::Loaded(library)
+                LoadResult::Loaded(library)
+            },
+        ))
     }
 
     fn update_extern_crate(&self, cnum: CrateNum, extern_crate: ExternCrate) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_incremental/src/persist/data.rs" "/checkout/compiler/rustc_incremental/src/persist/save.rs" "/checkout/compiler/rustc_metadata/src/creader.rs" "/checkout/compiler/rustc_incremental/src/persist/work_product.rs" "/checkout/compiler/rustc_metadata/src/dynamic_lib/tests.rs" "/checkout/compiler/rustc_incremental/src/persist/file_format.rs" "/checkout/compiler/rustc_incremental/src/persist/load.rs" "/checkout/compiler/rustc_metadata/src/locator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
