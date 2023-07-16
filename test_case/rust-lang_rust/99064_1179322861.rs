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
#####################                                                     30.2%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_typeck/src/check/method/suggest.rs at line 1040:
                         // Associated functions don’t take self as a parameter and
                         // they are not methods because they don’t have an instance of the struct to work with.
                         if def_kind == DefKind::AssocFn && lev_candidate.fn_has_self_parameter {
-                          err.span_suggestion(
-                              &format!(
-                                  "there is a method with a similar name",
-                              ),
-                              lev_candidate.name,
-                              lev_candidate.name,
-                              Applicability::MaybeIncorrect,
-                          );
+                            err.span_suggestion(
+                                span,
+                                &format!("there is a method with a similar name",),
+                                lev_candidate.name,
+                            );
                         } else {
-                          err.span_suggestion(
-                              span,
-                              span,
-                              &format!(
-                                  "there is {} {} with a similar name",
-                                  def_kind.article(),
-                                  def_kind.descr(lev_candidate.def_id),
-                              lev_candidate.name,
-                              Applicability::MaybeIncorrect,
-                          );
-                       }
-                       }
+                            err.span_suggestion(
+                                span,
+                                &format!(
+                                    "there is {} {} with a similar name",
+                                    def_kind.article(),
+                                    def_kind.descr(lev_candidate.def_id),
+                                lev_candidate.name,
+                                Applicability::MaybeIncorrect,
+                            );
+                        }
+                        }
                     }
                 }
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/rvalue_scopes.rs" "/checkout/compiler/rustc_typeck/src/check/inherited.rs" "/checkout/compiler/rustc_typeck/src/check/fallback.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_propagate.rs" "/checkout/compiler/rustc_typeck/src/check/method/suggest.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/arg_matrix.rs" "/checkout/compiler/rustc_typeck/src/check/method/prelude2021.rs" "/checkout/compiler/rustc_typeck/src/check/closure.rs"` failed.
Build completed unsuccessfully in 0:00:15
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
