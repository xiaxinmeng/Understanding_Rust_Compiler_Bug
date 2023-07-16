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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_expand/src/expand.rs at line 624:
     /// A macro's expansion does not fit in this fragment kind.
     /// For example, a non-type macro in a type position.
     fn error_wrong_fragment_kind(&mut self, kind: AstFragmentKind, mac: &ast::MacCall, span: Span) {
-        self.cx.emit_err(WrongFragmentKind {
-            span,
-            kind: kind.name(),
-            name: &mac.path,
-        });
+        self.cx.emit_err(WrongFragmentKind { span, kind: kind.name(), name: &mac.path });
 
         self.cx.trace_macros_diag();
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/mut_visit/tests.rs" "/checkout/compiler/rustc_expand/src/build.rs" "/checkout/compiler/rustc_expand/src/mbe.rs" "/checkout/compiler/rustc_expand/src/tests.rs" "/checkout/compiler/rustc_expand/src/placeholders.rs" "/checkout/compiler/rustc_expand/src/errors.rs" "/checkout/compiler/rustc_expand/src/expand.rs" "/checkout/compiler/rustc_driver/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
