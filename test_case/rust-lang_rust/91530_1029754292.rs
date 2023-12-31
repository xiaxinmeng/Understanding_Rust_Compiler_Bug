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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2044:
                         // If a tuple of length one was expected and the found expression has
                         // parentheses around it, perhaps the user meant to write `(expr,)` to
                         // build a tuple (issue #86100)
-                        (ty::Tuple(_), _) => self.emit_tuple_wrap_err(
-                            &mut err,
-                            span,
-                            expected,
-                        ),
-                        ),
+                        (ty::Tuple(_), _) => {
+                            self.emit_tuple_wrap_err(&mut err, span, found, expected)
                         // If a character was expected and the found expression is a string literal
                         // If a character was expected and the found expression is a string literal
                         // containing a single character, perhaps the user meant to write `'c'` to
                         // specify a character literal (issue #92479)
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 2140:
             err.multipart_suggestion(
                 msg,
-                vec![
-                vec![
-                    (span.shrink_to_lo(), "(".into()),
-                    (span.shrink_to_hi(), ",)".into()),
-                ],
+                vec![(span.shrink_to_lo(), "(".into()), (span.shrink_to_hi(), ",)".into())],
             );
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/equate.rs" "/checkout/compiler/rustc_infer/src/traits/project.rs" "/checkout/compiler/rustc_infer/src/infer/freshen.rs" "/checkout/compiler/rustc_infer/src/traits/util.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/nice_region_error/different_lifetimes.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
