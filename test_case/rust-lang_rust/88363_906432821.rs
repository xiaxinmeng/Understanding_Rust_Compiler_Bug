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
Diff in /checkout/compiler/rustc_span/src/source_map.rs at line 460:
         self.lookup_char_pos(sp.lo()).file.name.clone()
 
 
-    pub fn filename_for_diagnostics<'a>(
-        &self,
-        filename: &'a FileName,
-    ) -> FileNameDisplay<'a> {
+    pub fn filename_for_diagnostics<'a>(&self, filename: &'a FileName) -> FileNameDisplay<'a> {
         filename.display(self.path_mapping.filename_display_for_diagnostics)
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/hygiene.rs" "/checkout/compiler/rustc_span/src/symbol.rs" "/checkout/compiler/rustc_span/src/lev_distance/tests.rs" "/checkout/compiler/rustc_save_analysis/src/sig.rs" "/checkout/compiler/rustc_span/src/lev_distance.rs" "/checkout/compiler/rustc_save_analysis/src/span_utils.rs" "/checkout/compiler/rustc_span/src/source_map.rs" "/checkout/compiler/rustc_span/src/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
