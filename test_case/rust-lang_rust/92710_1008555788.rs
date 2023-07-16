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
Diff in /checkout/compiler/rustc_infer/src/infer/outlives/verify.rs at line 240:
         let c_b = self.param_env.caller_bounds();
         let param_bounds = self.collect_outlives_from_predicate_list(&compare_ty, c_b.into_iter());
-
-
         // Next, collect regions we scraped from the well-formedness
         // constraints in the fn signature. To do that, we walk the list
         // of known relations from the fn ctxt.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs" "/checkout/compiler/rustc_infer/src/infer/mod.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/verify.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/components.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/mod.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/env.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/obligations.rs" "/checkout/compiler/rustc_infer/src/infer/region_constraints/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
