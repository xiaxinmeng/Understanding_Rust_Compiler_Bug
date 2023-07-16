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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 2059:
     }
 }
 
-fn parse_pretty(
-    debugging_opts: &DebuggingOptions,
-    efmt: ErrorOutputType,
-) -> Option<PpMode> {
+fn parse_pretty(debugging_opts: &DebuggingOptions, efmt: ErrorOutputType) -> Option<PpMode> {
     use PpMode::*;
 
     let first = match debugging_opts.unpretty.as_deref()? {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_session/src/parse.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_session/src/search_paths.rs" "/checkout/compiler/rustc_session/src/output.rs" "/checkout/compiler/rustc_session/src/cgu_reuse_tracker.rs" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_session/src/options.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
