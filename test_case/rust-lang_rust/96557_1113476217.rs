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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/late.rs at line 3069:
 
     fn resolve_inline_const(&mut self, constant: &'ast AnonConst) {
         debug!("resolve_anon_const {constant:?}");
-        self.with_constant_rib(
-            IsRepeatExpr::No,
-            None,
-            |this| {
-                visit::walk_anon_const(this, constant);
-            },
-            },
-        );
+        self.with_constant_rib(IsRepeatExpr::No, true, None, |this| {
+            visit::walk_anon_const(this, constant);
     }
 
 
     fn resolve_expr(&mut self, expr: &'ast Expr, parent: Option<&'ast Expr>) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/creader.rs" "/checkout/compiler/rustc_resolve/src/imports.rs" "/checkout/compiler/rustc_metadata/src/native_libs.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_metadata/src/locator.rs" "/checkout/compiler/rustc_metadata/src/dependency_format.rs" "/checkout/compiler/rustc_graphviz/src/lib.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
