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
Diff in /checkout/compiler/rustc_ast_pretty/src/pprust/state.rs at line 349:
         self.comments().as_mut().and_then(|c| c.next())
 
-    fn maybe_print_trailing_comment(
-        &mut self,
-        span: rustc_span::Span,
-        span: rustc_span::Span,
-        next_pos: Option<BytePos>,
-    ) {
+    fn maybe_print_trailing_comment(&mut self, span: rustc_span::Span, next_pos: Option<BytePos>) {
         if let Some(cmnts) = self.comments() {
             if let Some(cmnt) = cmnts.trailing_comment(span, next_pos) {
                 self.print_comment(&cmnt);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/x86_64_pc_solaris.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_linux_android.rs" "/checkout/compiler/rustc_target/src/spec/netbsd_base.rs" "/checkout/compiler/rustc_target/src/spec/sparc_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/armv6k_nintendo_3ds.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7em_none_eabi.rs" "/checkout/compiler/rustc_target/src/spec/powerpc_wrs_vxworks.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/state.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
