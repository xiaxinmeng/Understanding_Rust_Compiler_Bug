plain
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
Diff in /checkout/src/librustdoc/config.rs at line 714:
         let with_examples = matches.opt_strs("with-examples");
         let call_locations = crate::scrape_examples::load_call_locations(with_examples, &diag)?;
-        let unstable_features = rustc_feature::UnstableFeatures::from_environment(
-            crate_name.as_deref(),
-        );
+        let unstable_features =
+        let unstable_features =
+            rustc_feature::UnstableFeatures::from_environment(crate_name.as_deref());
         let options = Options {
             input,
             proc_macro_crate,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/error.rs" "/checkout/src/librustdoc/fold.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/librustdoc/passes/strip_private.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/config.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
