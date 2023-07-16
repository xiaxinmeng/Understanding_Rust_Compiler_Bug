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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs at line 362:
             .infcx
             .probe(|_| self.match_projection_obligation_against_definition_bounds(obligation));
 
-        candidates.vec.extend(result.into_iter().map(|(idx, constness)| ProjectionCandidate(idx, constness)));
+        candidates
+            .vec
+            .extend(result.into_iter().map(|(idx, constness)| ProjectionCandidate(idx, constness)));
 
 
     /// Given an obligation like `<SomeTrait for T>`, searches the obligations that the caller
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/engine.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/eq.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
