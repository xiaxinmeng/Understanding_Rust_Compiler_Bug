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
Diff in /checkout/compiler/rustc_expand/src/mbe/macro_rules.rs at line 381:
             Failure(token, msg) => match self.best_failure {
                 Some((ref best_token, _, _)) if best_token.span.lo() >= token.span.lo() => {}
-                    self.best_failure =
-                    self.best_failure =
-                        Some((token.clone(), msg, self.remaining_matcher.expect("must have collected matcher already").clone()))
+                    self.best_failure = Some((
+                        msg,
+                        self.remaining_matcher
+                        self.remaining_matcher
+                            .expect("must have collected matcher already")
+                            .clone(),
                 }
             },
             },
             Error(err_sp, msg) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/mbe/macro_rules.rs" "/checkout/compiler/rustc_expand/src/mbe/transcribe.rs" "/checkout/compiler/rustc_expand/src/mbe/metavar_expr.rs" "/checkout/compiler/rustc_expand/src/mut_visit/tests.rs" "/checkout/compiler/rustc_expand/src/tokenstream/tests.rs" "/checkout/compiler/rustc_expand/src/mbe/quoted.rs" "/checkout/compiler/rustc_expand/src/proc_macro.rs" "/checkout/compiler/rustc_expand/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
