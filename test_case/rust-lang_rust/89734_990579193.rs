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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs at line 126:
             lifetime,
 
 
-        let (mention_influencer, influencer_point) = if sup_origin.span().overlaps(param.param_ty_span) {
-            // Account for `async fn` like in `async-await/issues/issue-62097.rs`.
-            // The desugaring of `async `fn`s causes `sup_origin` and `param` to point at the same
-            // place (but with different `ctxt`, hence `overlaps` instead of `==` above).
-            // This avoids the following:
-            //
-            //
-            // LL |     pub async fn run_dummy_fn(&self) {
-            //    |                               |
-            //    |                               |
-            //    |                               this data with an anonymous lifetime `'_`...
-            //    |                               ...is captured here...
-            (false, sup_origin.span())
-        } else {
-            (!sup_origin.span().overlaps(return_sp), param.param_ty_span)
-        };
+        let (mention_influencer, influencer_point) =
+            if sup_origin.span().overlaps(param.param_ty_span) {
+                // Account for `async fn` like in `async-await/issues/issue-62097.rs`.
+                // The desugaring of `async `fn`s causes `sup_origin` and `param` to point at the same
+                // place (but with different `ctxt`, hence `overlaps` instead of `==` above).
+                // This avoids the following:
+                //
+                //
+                // LL |     pub async fn run_dummy_fn(&self) {
+                //    |                               |
+                //    |                               |
+                //    |                               this data with an anonymous lifetime `'_`...
+                //    |                               ...is captured here...
+                (false, sup_origin.span())
+            } else {
+                (!sup_origin.span().overlaps(return_sp), param.param_ty_span)
+            };
         err.span_label(influencer_point, &format!("this data with {}...", lifetime));
 
         debug!("try_report_static_impl_trait: param_info={:?}", param);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/library/std/src/path/tests.rs" "/checkout/library/std/src/rt.rs" "/checkout/compiler/rustc_attr/src/builtin.rs" "/checkout/compiler/rustc_macros/src/symbols/tests.rs" "/checkout/compiler/rustc_macros/src/query.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/static_impl_trait.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
