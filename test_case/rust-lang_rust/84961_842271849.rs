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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_span/src/hygiene.rs at line 200:
     }
 
     pub fn with<T, F: FnOnce(&mut HygieneData) -> T>(f: F) -> T {
-        with_session_globals(|session_globals| {
-            f(&mut *session_globals.hygiene_data.borrow_mut())
-        })
+        with_session_globals(|session_globals| f(&mut *session_globals.hygiene_data.borrow_mut()))
 
 
     fn fresh_expn(&mut self, mut expn_data: Option<ExpnData>) -> ExpnId {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/symbol/tests.rs" "/checkout/compiler/rustc_span/src/fatal_error.rs" "/checkout/compiler/rustc_span/src/def_id.rs" "/checkout/compiler/rustc_save_analysis/src/dump_visitor.rs" "/checkout/compiler/rustc_span/src/caching_source_map_view.rs" "/checkout/compiler/rustc_save_analysis/src/sig.rs" "/checkout/compiler/rustc_span/src/analyze_source_file.rs" "/checkout/compiler/rustc_span/src/hygiene.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
