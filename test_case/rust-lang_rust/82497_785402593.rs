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
Diff in /checkout/src/librustdoc/config.rs at line 342:
         check_deprecated_options(&matches, &diag);
 
         // check for `--output-format=json`
-        if matches.opt_present("output-format") && !matches.opt_present("show-coverage") && !nightly_options::is_unstable_enabled(matches) {
+        if matches.opt_present("output-format")
+            && !matches.opt_present("show-coverage")
+            && !nightly_options::is_unstable_enabled(matches)
             rustc_session::early_error(
                 error_format,
                 error_format,
-                "the -Z unstable-options flag must be passed to enable --output-format for documentation generation"
+                "the -Z unstable-options flag must be passed to enable --output-format for documentation generation",
             );
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/job.rs" "/checkout/src/librustdoc/error.rs" "/checkout/src/bootstrap/metadata.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/librustdoc/clean/auto_trait.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
