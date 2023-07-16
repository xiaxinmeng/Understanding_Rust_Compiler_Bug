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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_errors/src/lib.rs at line 858:
     }
 
     pub fn has_errors(&self) -> Option<ErrorReported> {
-        if self.inner.borrow().has_errors() {
-            Some(ErrorReported(()))
-            None
-        }
-        }
+        if self.inner.borrow().has_errors() { Some(ErrorReported(())) } else { None }
     }
     pub fn has_errors_or_lint_errors(&self) -> bool {
         self.inner.borrow().has_errors_or_lint_errors()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs" "/checkout/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs" "/checkout/compiler/rustc_errors/src/lib.rs" "/checkout/compiler/rustc_save_analysis/src/dumper.rs" "/checkout/compiler/rustc_span/src/tests.rs" "/checkout/compiler/rustc_save_analysis/src/dump_visitor.rs" "/checkout/compiler/rustc_span/src/analyze_source_file.rs" "/checkout/compiler/rustc_errors/src/snippet.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
