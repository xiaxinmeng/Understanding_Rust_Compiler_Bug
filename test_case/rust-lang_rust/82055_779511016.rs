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
Diff in /checkout/compiler/rustc_typeck/src/astconv/generics.rs at line 65:
                 }),
                 }),
                 GenericParamDefKind::Const,
-            ) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-                match path.res {
-                    Res::Err => {
-                        add_braces_suggestion(arg, &mut err);
-                        err.set_primary_message(
-                            "unresolved item provided when a constant was expected",
-                        );
-                    }
-                    Res::Def(DefKind::TyParam, src_def_id) => (|| {
-                        let param_hir_id = match param.def_id.as_local() {
-                            Some(x) => tcx.hir().local_def_id_to_hir_id(x),
-                            None => return,
-                        };
-                        let param_name = tcx.hir().ty_param_name(param_hir_id);
-                        let param_type = tcx.type_of(param.def_id);
-                        let (appl, sugg_type) = if param_type.is_suggestable() {
-                            (Applicability::MaybeIncorrect, format!("{}", param_type))
-                        } else {
-                            (Applicability::HasPlaceholders, String::from("T"))
-                        err.span_suggestion(
-                        err.span_suggestion(
-                            tcx.def_span(src_def_id),
-                            "consider changing this type paramater to a `const`-generic",
-                            format!("const {}: {}", param_name, sugg_type),
-                            appl,
-                        );
-                    })(),
-                    _ => add_braces_suggestion(arg, &mut err),
+            ) => match path.res {
+                Res::Err => {
+                    add_braces_suggestion(arg, &mut err);
+                    err.set_primary_message(
+                        "unresolved item provided when a constant was expected",
+                    );
-            }
-            }
+                Res::Def(DefKind::TyParam, src_def_id) => (|| {
+                    let param_hir_id = match param.def_id.as_local() {
+                        Some(x) => tcx.hir().local_def_id_to_hir_id(x),
+                        None => return,
+                    };
+                    let param_name = tcx.hir().ty_param_name(param_hir_id);
+                    let param_type = tcx.type_of(param.def_id);
+                    let (appl, sugg_type) = if param_type.is_suggestable() {
+                        (Applicability::MaybeIncorrect, format!("{}", param_type))
+                    } else {
+                        (Applicability::HasPlaceholders, String::from("T"))
+                    err.span_suggestion(
+                    err.span_suggestion(
+                        tcx.def_span(src_def_id),
+                        "consider changing this type paramater to a `const`-generic",
+                        format!("const {}: {}", param_name, sugg_type),
+                        appl,
+                    );
+                })(),
+                _ => add_braces_suggestion(arg, &mut err),
             (
             (
                 GenericArg::Type(hir::Ty { kind: hir::TyKind::Path(_), .. }),
                 GenericParamDefKind::Const,
Build completed unsuccessfully in 0:00:17
