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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 2014:
         );
     }
 
-    if cg.profile_sample_use.is_some() && (cg.profile_generate.enabled() || cg.profile_use.is_some()) {
+    if cg.profile_sample_use.is_some()
+        && (cg.profile_generate.enabled() || cg.profile_use.is_some())
         early_error(
             error_format,
             error_format,
             "option `-C profile-sample-use` cannot be used with `-C profile-generate` or `-C profile-use`",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/util.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_infer/src/infer/mod.rs" "/checkout/compiler/rustc_infer/src/infer/combine.rs" "/checkout/compiler/rustc_session/src/config.rs"` failed.
Build completed unsuccessfully in 0:00:15
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
