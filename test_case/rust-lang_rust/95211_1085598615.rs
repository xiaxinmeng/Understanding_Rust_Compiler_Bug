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
Diff in /checkout/compiler/rustc_parse/src/parser/path.rs at line 280:
                         self.expect(&TokenKind::Comma)?;
                     } else {
                         self.expect_gt().map_err(|mut err| {
-                        // Attempt to find places where a missing `>` might belong.
-                        if let Some(arg) = args
-                            .iter()
-                            .rev()
-                            .skip_while(|arg| matches!(arg, AngleBracketedArg::Constraint(_)))
-                            .next()
-                            err.span_suggestion_verbose(
-                            err.span_suggestion_verbose(
-                                arg.span().shrink_to_hi(),
-                                "you might have meant to end the type parameters here",
-                                ">".to_string(),
-                            );
-                        }
-                        err
-                    })?;
-                    })?;
+                            // Attempt to find places where a missing `>` might belong.
+                            if let Some(arg) = args
+                                .iter()
+                                .rev()
+                                .skip_while(|arg| matches!(arg, AngleBracketedArg::Constraint(_)))
+                                .next()
+                                err.span_suggestion_verbose(
+                                err.span_suggestion_verbose(
+                                    arg.span().shrink_to_hi(),
+                                    "you might have meant to end the type parameters here",
+                                    ">".to_string(),
+                                );
+                            }
+                            err
+                        })?;
+                        })?;
                     }
                     let span = lo.to(self.prev_token.span);
                     AngleBracketedArgs { args, span }.into()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs" "/checkout/compiler/rustc_parse/src/parser/stmt.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_parse/src/parser/item.rs" "/checkout/compiler/rustc_parse/src/parser/diagnostics.rs" "/checkout/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs" "/checkout/compiler/rustc_parse/src/parser/ty.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
