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
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 3014:
                     (trait_span.shrink_to_hi(), format!("{}>", self_name,)),
                     (self_ty.span, "T".to_owned()),
                 ];
-                diagnostic
-                    .multipart_suggestion(blanket_msg, blanket_sugg, Applicability::Unspecified);
+                diagnostic.multipart_suggestion(
+                    blanket_msg,
+                    blanket_sugg,
+                    Applicability::Unspecified,
             }
         }
     }
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 3057:
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 3057:
                 let label = "add `dyn` keyword before this trait";
                 let mut diag =
                     rustc_errors::struct_span_err!(tcx.sess, self_ty.span, E0782, "{}", msg);
-                diag
-                    .multipart_suggestion_verbose(label, sugg, Applicability::MachineApplicable);
+                diag.multipart_suggestion_verbose(label, sugg, Applicability::MachineApplicable);
                 self.maybe_lint_blanket_trait_impl(&self_ty, &mut diag, needs_bracket);
                 diag.emit();
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 3070:
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 3070:
                     |lint| {
                         let mut diag = lint.build(msg);
                         diag.multipart_suggestion_verbose(
-                                "use `dyn`",
-                                sugg,
-                            );
-                            );
-                        self.maybe_lint_blanket_trait_impl::<()>(&self_ty, &mut diag, needs_bracket);
+                            "use `dyn`",
+                            sugg,
+                        );
+                        );
+                        self.maybe_lint_blanket_trait_impl::<()>(
+                            &self_ty,
+                            &mut diag,
+                            needs_bracket,
+                        );
                         diag.emit();
                 );
                 );
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/compiler/rustc_typeck/src/constrained_generic_params.rs" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs" "/checkout/compiler/rustc_typeck/src/variance/mod.rs" "/checkout/compiler/rustc_typeck/src/outlives/outlives_bounds.rs" "/checkout/compiler/rustc_typeck/src/variance/constraints.rs" "/checkout/compiler/rustc_lint_defs/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
