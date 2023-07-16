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
Diff in /checkout/compiler/rustc_errors/src/diagnostic.rs at line 940:
         self
     }
 
-    pub fn replace_args(&mut self, args: FxHashMap<DiagnosticArgName<'static>, DiagnosticArgValue<'static>>) {
+        &mut self,
+        &mut self,
+        args: FxHashMap<DiagnosticArgName<'static>, DiagnosticArgValue<'static>>,
+    ) {
         self.args = args;
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/evaluate_obligation.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_errors/src/json/tests.rs" "/checkout/compiler/rustc_error_codes/src/error_codes.rs" "/checkout/compiler/rustc_borrowck/src/borrowck_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs" "/checkout/compiler/rustc_borrowck/src/renumber.rs" "/checkout/compiler/rustc_errors/src/diagnostic.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
