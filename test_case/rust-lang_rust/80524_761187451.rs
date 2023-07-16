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
Diff in /checkout/compiler/rustc_lint/src/context.rs at line 369:
                             // 2. The tool isn't currently running, so no lints will be registered.
                             // To avoid giving a false positive, ignore all unknown lints.
                             CheckLintNameResult::Tool(Err((None, String::new())))
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/context.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                        };
                     }
                     }
                     Some(LintGroup { lint_ids, .. }) => {
                         return CheckLintNameResult::Tool(Ok(&lint_ids));
Diff in /checkout/compiler/rustc_lint/src/context.rs at line 412:
 
 
     fn no_lint_suggestion(&self, lint_name: &str) -> CheckLintNameResult<'_> {
-        let symbols =
-            self.by_name.keys().map(|name| Symbol::intern(&name)).collect::<Vec<_>>();
+        let symbols = self.by_name.keys().map(|name| Symbol::intern(&name)).collect::<Vec<_>>();
-        let suggestion = find_best_match_for_name(
-            &symbols,
-            &symbols,
-            Symbol::intern(&lint_name.to_lowercase()),
-            None,
-        );
+        let suggestion =
+            find_best_match_for_name(&symbols, Symbol::intern(&lint_name.to_lowercase()), None);
 
         CheckLintNameResult::NoLint(suggestion)
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:21
