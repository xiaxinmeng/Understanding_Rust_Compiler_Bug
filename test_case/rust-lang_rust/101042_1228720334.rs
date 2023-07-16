plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/region_name.rs at line 790:
         let opaque_ty = hir.item(id);
         if let hir::ItemKind::OpaqueTy(hir::OpaqueTy {
             bounds:
-                [hir::GenericBound::LangItemTrait(
-                    hir::LangItem::Future,
-                    _,
-                    _,
-                    hir::GenericArgs {
-                            [hir::TypeBinding {
-                            [hir::TypeBinding {
-                                ident: Ident { name: sym::Output, .. },
-                                kind: hir::TypeBindingKind::Equality { term: hir::Term::Ty(ty) },
-                            }],
-                        ..
-                    },
-                )],
-                )],
+                [
+                    hir::GenericBound::LangItemTrait(
+                        hir::LangItem::Future,
+                        _,
+                        _,
+                        hir::GenericArgs {
+                                [
+                                    hir::TypeBinding {
+                                    hir::TypeBinding {
+                                        ident: Ident { name: sym::Output, .. },
+                                        kind:
+                                            hir::TypeBindingKind::Equality { term: hir::Term::Ty(ty) },
+                                    },
+                                ],
+                            ..
+                        },
+                        },
+                    ),
+                ],
             ..
         }) = opaque_ty.kind
         {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/diagnostics/mod.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/move_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/region_name.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/var_name.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_borrowck/src/prefixes.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
