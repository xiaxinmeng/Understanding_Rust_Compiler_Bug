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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 627:
                         Issue::Invalid(arg) => {
                             let span = provided_args[arg].span;
                             let expected_type = expected_input_tys[arg];
-                            let found_type = final_arg_types.iter().filter(|(i, _, _)| *i == arg).next().unwrap().1;
-                            labels.push((span, format!("expected {}, found {}", expected_type, found_type)));
+                            let found_type = final_arg_types
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                                .iter()
+                                .filter(|(i, _, _)| *i == arg)
+                                .next()
+                                .unwrap()
+                                .1;
+                            labels.push((
+                                span,
+                                format!("expected {}, found {}", expected_type, found_type),
+                            ));
                         }
                         Issue::Extra(arg) => {
                             // FIXME: This could probably be a lot cleaner, but I dunno how
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 635:
                             let span = provided_args[arg].span;
                             let hungry_span = if arg < provided_args.len() - 1 {
                                 // Eat the comma
-                                    span.lo(),
-                                    span.lo(),
-                                    span.hi() + BytePos(2),
-                                    span.ctxt(),
-                                )
+                                Span::new(span.lo(), span.hi() + BytePos(2), span.ctxt())
                                 span
                             };
                             };
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 650:
                                 .map(|ident| format!("fn {}(..)", ident))
                                 .unwrap_or("this function".to_string());
 
-                            labels.push((span, format!("no parameter of type {} is needed in {}", found_type, fn_name)));
+                            labels.push((
+                                span,
+                                format!(
+                                    "no parameter of type {} is needed in {}",
+                                    found_type, fn_name
+                            ));
+                            ));
                             if issue_count > 1 {
                                 suggestions.push((hungry_span, format!("")));
                             }
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 677:
                             };
                             };
                             let expected_type = expected_input_tys[arg];
-                            labels.push((missing_span, format!("missing argument of type {}", expected_type)));
+                            labels.push((
+                                missing_span,
+                                format!("missing argument of type {}", expected_type),
+                            ));
                             if issue_count > 1 {
-                                suggestions.push((missing_span, format!(" {{{}}},", expected_type)));
+                                suggestions
+                                    .push((missing_span, format!(" {{{}}},", expected_type)));
                         }
                         }
                         Issue::Swap(arg, other) => {
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 687:
                             let second_span = provided_args[other].span;
                             let first_snippet = source_map.span_to_snippet(first_span).unwrap();
                             let second_snippet = source_map.span_to_snippet(second_span).unwrap();
-                            let expected_types = (expected_input_tys[arg], expected_input_tys[other]);
+                            let expected_types =
+                                (expected_input_tys[arg], expected_input_tys[other]);
                             let found_types = (
-                                final_arg_types.iter().filter(|(i, _, _)| *i == arg).next().unwrap().1,
-                                final_arg_types.iter().filter(|(i, _, _)| *i == other).next().unwrap().1,
+                                final_arg_types
+                                    .iter()
+                                    .filter(|(i, _, _)| *i == arg)
+                                    .next()
+                                    .unwrap()
+                                    .1,
+                                final_arg_types
+                                    .iter()
+                                    .filter(|(i, _, _)| *i == other)
+                                    .next()
+                                    .unwrap()
+                                    .1,
                             );
-                            labels.push((first_span, format!("expected {}, found {}", expected_types.0, found_types.0)));
+                            labels.push((
+                                first_span,
+                                format!("expected {}, found {}", expected_types.0, found_types.0),
+                            ));
                             suggestions.push((first_span, second_snippet));
-                            labels.push((second_span, format!("expected {}, found {}", expected_types.1, found_types.1)));
+                            labels.push((
+                                second_span,
+                                format!("expected {}, found {}", expected_types.1, found_types.1),
+                            ));
                             suggestions.push((second_span, first_snippet));
                         }
                         Issue::Permutation(args) => {
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 704:
                                     let dst_span = provided_args[dst].span;
                                     let snippet = source_map.span_to_snippet(src_span).unwrap();
                                     let expected_type = expected_input_tys[dst];
-                                    let found_type = final_arg_types.iter().filter(|(i, _, _)| *i == dst).next().unwrap().1;
-                                    labels.push((dst_span, format!("expected {}, found {}", expected_type, found_type)));
+                                    let found_type = final_arg_types
+                                        .iter()
+                                        .filter(|(i, _, _)| *i == dst)
+                                        .next()
+                                        .unwrap()
+                                        .1;
+                                    labels.push((
+                                        dst_span,
+                                        format!("expected {}, found {}", expected_type, found_type),
+                                    ));
                                     suggestions.push((dst_span, snippet));
                             }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:19
