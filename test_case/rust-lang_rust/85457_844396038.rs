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
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 794:
                                     if i_meta.has_name(sym::include) {
                                         if let Some(value) = i_meta.value_str() {
                                             // if there are multiple attributes, the suggestion would suggest deleting all of them, which is incorrect
-                                            let applicability = if list.len() == 1 { Applicability::MachineApplicable } else { Applicability::MaybeIncorrect };
-                                            let inner = if attr.style == AttrStyle::Inner { "!" } else { "" };
+                                            let applicability = if list.len() == 1 {
+                                            } else {
+                                                Applicability::MaybeIncorrect
+                                            };
+                                            };
+                                            let inner = if attr.style == AttrStyle::Inner {
+                                                "!"
+                                                ""
+                                            };
                                             diag.span_suggestion(
                                             diag.span_suggestion(
                                                 attr.meta().unwrap().span,
                                                 "use `doc = include_str!` instead",
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 802:
-                                                format!("#{}[doc = include_str!(\"{}\")]", inner, value),
+                                                format!(
+                                                    "#{}[doc = include_str!(\"{}\")]",
+                                                    inner, value
                                                 applicability,
                                             );
                                         }
                                         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir/src/arena.rs" "/checkout/compiler/rustc_passes/src/liveness/rwu_table.rs" "/checkout/compiler/rustc_passes/src/intrinsicck.rs" "/checkout/compiler/rustc_hir/src/definitions.rs" "/checkout/compiler/rustc_passes/src/diagnostic_items.rs" "/checkout/compiler/rustc_passes/src/hir_stats.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs" "/checkout/compiler/rustc_passes/src/reachable.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
