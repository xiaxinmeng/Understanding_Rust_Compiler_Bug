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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 2067:
         _ => {}
     }
 
-    if cg.instrument_coverage.is_some()
-        && cg.instrument_coverage != Some(InstrumentCoverage::Off)
-    {
+    if cg.instrument_coverage.is_some() && cg.instrument_coverage != Some(InstrumentCoverage::Off) {
         if cg.profile_generate.enabled() || cg.profile_use.is_some() {
             early_error(
                 error_format,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_session/src/filesearch.rs" "/checkout/compiler/rustc_session/src/output.rs" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_session/src/options.rs" "/checkout/compiler/rustc_session/src/parse.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_session/src/cstore.rs" "/checkout/compiler/rustc_save_analysis/src/dump_visitor.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
