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
Diff in /checkout/compiler/rustc_typeck/src/astconv/generics.rs at line 7:
 use rustc_errors::{pluralize, struct_span_err, Applicability, DiagnosticId, ErrorReported};
 use rustc_hir as hir;
 use rustc_hir::def_id::DefId;
-use rustc_hir::{GenericArg};
+use rustc_hir::GenericArg;
 use rustc_middle::ty::{
     self, subst, subst::SubstsRef, GenericParamDef, GenericParamDefKind, Ty, TyCtxt,
 };
Diff in /checkout/compiler/rustc_typeck/src/astconv/generics.rs at line 61:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
         // Specific suggestion set for diagnostics
         if let Some(param) = param {
-          match (arg, &param.kind) {
-              (
-                  GenericArg::Type(hir::Ty { kind: hir::TyKind::Path { .. }, .. }),
-                  GenericParamDefKind::Const { .. },
-              ) => {
-                  let suggestions = vec![
-                      (arg.span().shrink_to_lo(), String::from("{ ")),
-                      (arg.span().shrink_to_hi(), String::from(" }")),
-                  ];
-                  err.multipart_suggestion(
-                      "if this generic argument was intended as a const parameter, \
+            match (arg, &param.kind) {
+                (
+                    GenericArg::Type(hir::Ty { kind: hir::TyKind::Path { .. }, .. }),
+                    GenericParamDefKind::Const { .. },
+                ) => {
+                    let suggestions = vec![
+                        (arg.span().shrink_to_lo(), String::from("{ ")),
+                        (arg.span().shrink_to_hi(), String::from(" }")),
+                    ];
+                    err.multipart_suggestion(
+                        "if this generic argument was intended as a const parameter, \
                     try surrounding it with braces:",
-                      suggestions,
-                      Applicability::MaybeIncorrect,
-                  );
-              (
-              (
-                  GenericArg::Type(hir::Ty { kind: hir::TyKind::Array(_, len), .. }),
-                  GenericParamDefKind::Const { .. },
-              ) => if tcx.type_of(param.def_id).is_ptr_sized_integral() {
-                  let snippet = sess
-                      .source_map()
-                      .span_to_snippet(tcx.hir().span(len.hir_id))
-                      .unwrap_or(String::from("N"));
-                  err.span_suggestion(
-                      arg.span(),
-                      "`[T; N]` provided where `usize` was expected, try",
-                      format!("{}", snippet),
-                      Applicability::MaybeIncorrect,
-                  );
-              _ => {}
-          }
+                        suggestions,
+                        suggestions,
+                        Applicability::MaybeIncorrect,
+                    );
+                (
+                (
+                    GenericArg::Type(hir::Ty { kind: hir::TyKind::Array(_, len), .. }),
+                    GenericParamDefKind::Const { .. },
+                ) => {
+                    if tcx.type_of(param.def_id).is_ptr_sized_integral() {
+                        let snippet = sess
+                            .source_map()
+                            .span_to_snippet(tcx.hir().span(len.hir_id))
+                            .unwrap_or(String::from("N"));
+                        err.span_suggestion(
+                            arg.span(),
+                            "`[T; N]` provided where `usize` was expected, try",
+                            format!("{}", snippet),
+                            Applicability::MaybeIncorrect,
+                        );
+                }
+                _ => {}
+            }
         }
         }
 
         // This note is only true when generic parameters are strictly ordered by their kind.
Build completed unsuccessfully in 0:00:14
