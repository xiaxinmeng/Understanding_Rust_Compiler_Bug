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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/mod.rs at line 2391:
     ///
     /// When `-Zdebug-macros` is provided then debuginfo will never be collapsed.
     pub fn should_collapse_debuginfo(self, span: Span) -> bool {
-        !self.sess.opts.unstable_opts.debug_macros &&
-            if self.features().collapse_debuginfo {
+        !self.sess.opts.unstable_opts.debug_macros
+            && if self.features().collapse_debuginfo {
                 span.in_macro_expansion_with_collapse_debuginfo()
                 span.from_expansion()
                 span.from_expansion()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/terminator.rs" "/checkout/compiler/rustc_middle/src/ty/mod.rs" "/checkout/compiler/rustc_middle/src/ty/generics.rs" "/checkout/compiler/rustc_middle/src/mir/syntax.rs" "/checkout/compiler/rustc_middle/src/mir/type_visitable.rs" "/checkout/compiler/rustc_middle/src/ty/instance.rs" "/checkout/compiler/rustc_middle/src/mir/visit.rs" "/checkout/compiler/rustc_middle/src/ty/relate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
