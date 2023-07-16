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
Diff in /checkout/compiler/rustc_expand/src/mbe/macro_parser.rs at line 686:
                         eof_item.matches.iter_mut().map(|dv| Lrc::make_mut(dv).pop().unwrap());
                     nameize(parser.sess, ms, matches)
                 }
-                EofItems::Multiple =>
-                        parser.token.span,
-                        parser.token.span,
-                        "ambiguity: multiple successful parses".to_string(),
+                EofItems::Multiple => {
+                    Error(parser.token.span, "ambiguity: multiple successful parses".to_string())
+                }
+                EofItems::None => Failure(
+                        token::Eof,
+                        token::Eof,
+                        if parser.token.span.is_dummy() {
+                            parser.token.span
+                        } else {
+                            parser.token.span.shrink_to_hi()
                     ),
                     ),
-                EofItems::None =>
-                    Failure(
-                            token::Eof,
-                            token::Eof,
-                            if parser.token.span.is_dummy() {
-                                parser.token.span
-                            } else {
-                                parser.token.span.shrink_to_hi()
-                        ),
-                        "missing tokens in macro arguments",
-                    ),
-            }
-            }
+                    "missing tokens in macro arguments",
+                ),
+            };
         }
         // Performance hack: `eof_items` may share matchers via `Rc` with other things that we want
         // to modify. Dropping `eof_items` now may drop these refcounts to 1, preventing an
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/tests.rs" "/checkout/compiler/rustc_hir/src/stable_hash_impls.rs" "/checkout/compiler/rustc_expand/src/module.rs" "/checkout/compiler/rustc_hir/src/pat_util.rs" "/checkout/compiler/rustc_expand/src/proc_macro.rs" "/checkout/compiler/rustc_expand/src/mbe/quoted.rs" "/checkout/compiler/rustc_expand/src/mbe/macro_parser.rs" "/checkout/compiler/rustc_hir/src/tests.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
