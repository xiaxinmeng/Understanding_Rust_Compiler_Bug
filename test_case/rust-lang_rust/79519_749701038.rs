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
Diff in /checkout/compiler/rustc_ast_lowering/src/expr.rs at line 524:
         });
         let hir_id = self.next_id();
         self.lower_attrs(hir_id, &arm.attrs);
-        hir::Arm {
-            hir_id,
-            pat,
-            guard,
-            body: self.lower_expr(&arm.body),
-            span: arm.span,
-        }
+        hir::Arm { hir_id, pat, guard, body: self.lower_expr(&arm.body), span: arm.span }
 
 
     /// Lower an `async` construct to a generator that is then wrapped so it implements `Future`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_lowering/src/expr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
