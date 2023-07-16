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
Diff in /checkout/compiler/rustc_typeck/src/check/pat.rs at line 1491:
                 if let Some(suggested_name) = suggested_name {
                     err.span_suggestion_hide_inline(
                         ident.span,
-                        &format!(
-                            "did you mean `{}`? (a similarly named field)",
-                            suggested_name
-                        ),
+                        &format!("did you mean `{}`? (a similarly named field)", suggested_name),
                         suggested_name.to_string(),
                     );
                     );
Diff in /checkout/compiler/rustc_typeck/src/check/method/suggest.rs at line 983:
                     ) {
                         err.span_suggestion_hide_inline(
                             span,
-                            &format!(
-                                "did you mean `{}`? (a similarly named variant)",
-                                suggestion
-                            ),
+                            &format!("did you mean `{}`? (a similarly named variant)", suggestion),
                             suggestion.to_string(),
                         );
                         );
Diff in /checkout/compiler/rustc_typeck/src/astconv/errors.rs at line 186:
         ) {
             err.span_suggestion_hide_inline(
                 assoc_name.span,
-                &format!(
-                    "did you mean `{}`? (a similarly named associated type)",
-                    suggested_name
-                ),
+                &format!("did you mean `{}`? (a similarly named associated type)", suggested_name),
                 suggested_name.to_string(),
             );
             );
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/place_op.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/check/mod.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/method/probe.rs" "/checkout/compiler/rustc_typeck/src/check/method/confirm.rs" "/checkout/compiler/rustc_typeck/src/check/method/prelude2021.rs" "/checkout/compiler/rustc_typeck/src/check/expectation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
