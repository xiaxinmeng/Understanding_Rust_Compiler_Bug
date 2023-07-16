plain
Successfully built 228b75302712
Successfully tagged rust-ci:latest
Built container sha256:228b753027128b8e6c5f9b25850109c6ae2f4cf7e78d20c929c32bb7dd7c9927
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_save_analysis/src/lib.rs at line 967:
     with_no_trimmed_paths!(tcx.dep_graph.with_ignore(|| {
         info!("Dumping crate {}", cratename);
 
-            // Privacy checking must be done outside of type inference; use a
-            // fallback in case effective visibilities couldn't have been correctly computed.
-            let effective_visibilities = match tcx.sess.compile_status() {
-                Ok(..) => tcx.effective_visibilities(()),
-                Err(..) => tcx.arena.alloc(EffectiveVisibilities::default()),
-            };
+        // Privacy checking must be done outside of type inference; use a
+        // fallback in case effective visibilities couldn't have been correctly computed.
+        let effective_visibilities = match tcx.sess.compile_status() {
+            Ok(..) => tcx.effective_visibilities(()),
+            Err(..) => tcx.arena.alloc(EffectiveVisibilities::default()),
 
 
-            let save_ctxt = SaveContext {
-                tcx,
-                maybe_typeck_results: None,
-                effective_visibilities: &effective_visibilities,
-                span_utils: SpanUtils::new(&tcx.sess),
-                config: find_config(config),
-                impl_counter: Cell::new(0),
-            };
+        let save_ctxt = SaveContext {
+            tcx,
+            maybe_typeck_results: None,
+            effective_visibilities: &effective_visibilities,
+            span_utils: SpanUtils::new(&tcx.sess),
+            config: find_config(config),
+            impl_counter: Cell::new(0),
 
 
         let mut visitor = DumpVisitor::new(save_ctxt);
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/vec_map/tests.rs" "/checkout/compiler/rustc_data_structures/src/small_str.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map.rs" "/checkout/compiler/rustc_data_structures/src/sync.rs" "/checkout/compiler/rustc_data_structures/src/base_n/tests.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map/index_map.rs" "/checkout/compiler/rustc_save_analysis/src/lib.rs" "/checkout/compiler/rustc_data_structures/src/tiny_list.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
