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
Diff in /checkout/compiler/rustc_macros/src/diagnostics/diagnostic_builder.rs at line 5:
     DiagnosticDeriveError,
 };
 use crate::diagnostics::utils::{
-    report_error_if_not_applied_to_span, report_type_error, type_is_unit,
-    type_matches_path, Applicability, FieldInfo, FieldInnerTy, HasFieldMap, SetOnce,
+    report_error_if_not_applied_to_span, report_type_error, type_is_unit, type_matches_path,
+    Applicability, FieldInfo, FieldInnerTy, HasFieldMap, SetOnce,
 };
 use proc_macro2::{Ident, TokenStream};
 use quote::{format_ident, quote};
Diff in /checkout/compiler/rustc_macros/src/diagnostics/diagnostic_builder.rs at line 396:
                 Ok(self.add_spanned_subdiagnostic(binding, ident, msg))
             }
             "note" | "help" if type_is_unit(&info.ty) => Ok(self.add_subdiagnostic(ident, msg)),
-            "note" | "help" => {
-                report_type_error(attr, "`Span` or `()`")?
-            }
+            "note" | "help" => report_type_error(attr, "`Span` or `()`")?,
             _ => unreachable!(),
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_macros/src/symbols.rs" "/checkout/compiler/rustc_macros/src/diagnostics/fluent.rs" "/checkout/compiler/rustc_resolve/src/imports.rs" "/checkout/compiler/rustc_macros/src/diagnostics/utils.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_macros/src/diagnostics/diagnostic_builder.rs" "/checkout/compiler/rustc_resolve/src/late/diagnostics.rs" "/checkout/compiler/rustc_macros/src/symbols/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
