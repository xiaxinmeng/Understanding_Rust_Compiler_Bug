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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 1555:
     }
 }
 
-fn select_debuginfo(
-    matches: &getopts::Matches,
-    cg: &CodegenOptions,
-) -> DebugInfo {
+fn select_debuginfo(matches: &getopts::Matches, cg: &CodegenOptions) -> DebugInfo {
     let max_g = matches.opt_positions("g").into_iter().max();
     let max_c = matches
         .opt_strs_pos("C")
Diff in /checkout/compiler/rustc_session/src/config.rs at line 1568:
             if let Some("debuginfo") = s.splitn(2, '=').next() { Some(i) } else { None }
         .max();
         .max();
-    if max_g > max_c {
-        DebugInfo::Full
-    } else {
-        cg.debuginfo
-    }
+    if max_g > max_c { DebugInfo::Full } else { cg.debuginfo }
 
 fn parse_libs(
Diff in /checkout/compiler/rustc_interface/src/tests.rs at line 8:
Diff in /checkout/compiler/rustc_interface/src/tests.rs at line 8:
 use rustc_session::config::{rustc_optgroups, ErrorOutputType, ExternLocation, Options, Passes};
 use rustc_session::config::{CFGuard, ExternEntry, LinkerPluginLto, LtoCli, SwitchWithOptPath};
 use rustc_session::config::{
-    Externs, DebugInfo, OutputType, OutputTypes, SymbolManglingVersion, WasiExecModel,
+    DebugInfo, Externs, OutputType, OutputTypes, SymbolManglingVersion, WasiExecModel,
 use rustc_session::lint::Level;
 use rustc_session::search_paths::SearchPath;
 use rustc_session::search_paths::SearchPath;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_session/src/lib.rs" "/checkout/compiler/rustc_session/src/session.rs" "/checkout/compiler/rustc_session/src/search_paths.rs" "/checkout/compiler/rustc_session/src/utils.rs" "/checkout/compiler/rustc_session/src/cgu_reuse_tracker.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_session/src/output.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
