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
Diff in /checkout/compiler/rustc_borrowck/src/nll.rs at line 189:
             move_data,
             elements,
             upvars,
-            use_polonius
+            use_polonius,
 
 
     if let Some(all_facts) = &mut all_facts {
Diff in /checkout/compiler/rustc_borrowck/src/type_check/mod.rs at line 188:
         &mut borrowck_context,
         |mut cx| {
             cx.equate_inputs_and_outputs(&body, universal_regions, &normalized_inputs_and_output);
-            liveness::generate(&mut cx, body, elements, flow_inits, move_data, location_table, use_polonius);
+            liveness::generate(
+                &mut cx,
+                body,
+                elements,
+                flow_inits,
+                move_data,
+                use_polonius,
+            );
 
 
             translate_outlives_facts(&mut cx);
             let opaque_type_values = mem::take(&mut infcx.inner.borrow_mut().opaque_types);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/type_check/free_region_relations.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs" "/checkout/compiler/rustc_borrowck/src/type_check/mod.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/var_name.rs" "/checkout/compiler/rustc_borrowck/src/type_check/relate_tys.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/bound_region_errors.rs" "/checkout/compiler/rustc_borrowck/src/type_check/canonical.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/region_name.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
