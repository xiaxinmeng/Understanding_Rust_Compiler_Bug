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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 885:
     user_cfg
 }
 
-pub fn build_target_config(opts: &Options, target_override: Option<Target>, sysroot: &PathBuf) -> Target {
-    let target_result = target_override.map_or_else(|| Target::search(&opts.target_triple, sysroot), Ok);
+pub fn build_target_config(
+    opts: &Options,
+    target_override: Option<Target>,
+    sysroot: &PathBuf,
+) -> Target {
+    let target_result =
+        target_override.map_or_else(|| Target::search(&opts.target_triple, sysroot), Ok);
     let target = target_result.unwrap_or_else(|e| {
         early_error(
             opts.error_format,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lexer/src/lib.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_session/src/search_paths.rs" "/checkout/compiler/rustc_session/src/session.rs" "/checkout/compiler/rustc_session/src/filesearch.rs" "/checkout/compiler/rustc_session/src/utils.rs" "/checkout/compiler/rustc_session/src/cgu_reuse_tracker.rs" "/checkout/compiler/rustc_session/src/config.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
