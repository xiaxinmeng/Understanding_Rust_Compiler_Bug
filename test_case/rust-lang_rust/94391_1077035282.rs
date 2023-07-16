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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs at line 1117:
             match self.query_mode {
                 TraitQueryMode::Standard => {
                     if self.infcx.is_tainted_by_errors() {
-                        return Err(OverflowError::Error(ErrorGuaranteed::unchecked_claim_error_was_emitted()));
+                        return Err(OverflowError::Error(
+                            ErrorGuaranteed::unchecked_claim_error_was_emitted(),
                     }
                     }
                     self.infcx.report_overflow_error(error_obligation, true);
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs" "/checkout/compiler/rustc_builtin_macros/src/proc_macro_harness.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs" "/checkout/compiler/rustc_builtin_macros/src/test_harness.rs" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs" "/checkout/compiler/rustc_builtin_macros/src/concat.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
