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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_hir_analysis/src/astconv/mod.rs at line 2409:
                 }
             }
             Res::Def(
-                DefKind::Enum
-                | DefKind::Struct
-                | DefKind::Union
-                | DefKind::ForeignTy,
+                DefKind::Enum | DefKind::Struct | DefKind::Union | DefKind::ForeignTy,
                 did,
             ) => {
                 assert_eq!(opt_self_ty, None);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/coherence/mod.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/generics.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/stmt.rs" "/checkout/compiler/rustc_hir_analysis/src/collect.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/mod.rs" "/checkout/compiler/rustc_hir_analysis/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
