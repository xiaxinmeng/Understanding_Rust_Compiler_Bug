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
Diff in /checkout/compiler/rustc_hir_analysis/src/check/region.rs at line 259:
                 // being dropped last. For the Let exception, see below.
                 let terminate_lhs = match l.kind {
                     hir::ExprKind::Let(_) => false,
-                    hir::ExprKind::Binary(source_map::Spanned { node, .. }, ..) if node == outer => false,
+                    hir::ExprKind::Binary(source_map::Spanned { node, .. }, ..)
+                        if node == outer =>
+                        false
+                    }
                     _ => true,
                 };
                 };
                 if terminate_lhs {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/check/dropck.rs" "/checkout/compiler/rustc_hir_analysis/src/check/intrinsicck.rs" "/checkout/compiler/rustc_hir_analysis/src/check/upvar.rs" "/checkout/compiler/rustc_hir_analysis/src/check/region.rs" "/checkout/compiler/rustc_hir_analysis/src/check/pat.rs" "/checkout/compiler/rustc_hir_analysis/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_hir_analysis/src/check/_match.rs" "/checkout/compiler/rustc_hir_analysis/src/check/expectation.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
