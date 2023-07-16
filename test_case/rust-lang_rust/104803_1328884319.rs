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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs at line 332:
             if candidates.vec.is_empty() {
                 self.assemble_candidates_from_auto_impls(obligation, &mut candidates);
             }
-            if self.tcx().features().impl_unified_exhaustive_const_traits && candidates.vec.is_empty() {
-
+            if self.tcx().features().impl_unified_exhaustive_const_traits
+                && candidates.vec.is_empty()
+            {
                 candidates.vec.push(LazyCandidate(obligation.predicate));
-
         }
         Ok(candidates)
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/wf.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/fulfill.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs" "/checkout/compiler/rustc_trait_selection/src/traits/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
