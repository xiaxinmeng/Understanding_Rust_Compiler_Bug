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
         }
     }
 
-
     let mut disr_vals: Vec<Discr<'tcx>> = Vec::with_capacity(vs.len());
     // This tracks the previous variant span (in the loop) incase we need it for diagnostics
     let mut prev_variant_span: Span = Span::default();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_ast_passes/src/feature_gate.rs" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs" "/checkout/compiler/rustc_ast_passes/src/ast_validation.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsicck.rs" "/checkout/compiler/rustc_typeck/src/check/regionck.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
