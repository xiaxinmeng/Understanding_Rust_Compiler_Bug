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
Diff in /checkout/compiler/rustc_infer/src/infer/higher_ranked/mod.rs at line 45:
             debug!("a_prime={:?}", a_prime);
             debug!("b_prime={:?}", b_prime);
 
-            let old_trace = if let Some(new_trace) = T::to_sub_trace(self.tcx(), &self.trace.cause, a_is_expected, a_prime, b_prime) {
+            let old_trace = if let Some(new_trace) =
+                T::to_sub_trace(self.tcx(), &self.trace.cause, a_is_expected, a_prime, b_prime)
                 Some(std::mem::replace(&mut self.trace, new_trace))
             } else {
                 None
                 None
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/higher_ranked/mod.rs" "/checkout/compiler/rustc_infer/src/infer/lattice.rs" "/checkout/compiler/rustc_infer/src/infer/undo_log.rs" "/checkout/compiler/rustc_infer/src/infer/at.rs" "/checkout/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs" "/checkout/compiler/rustc_infer/src/infer/region_constraints/mod.rs" "/checkout/compiler/rustc_infer/src/infer/canonical/query_response.rs" "/checkout/compiler/rustc_infer/src/infer/fudge.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
