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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 307:
 
         // If there is no expectation, expect formal_input_tys.
-        let expected_input_tys =
-        let expected_input_tys =
-            if !expected_input_tys.is_empty() { expected_input_tys } else { formal_input_tys.clone() };
+        let expected_input_tys = if !expected_input_tys.is_empty() {
+            expected_input_tys
+        } else {
+            formal_input_tys.clone()
 
 
         assert_eq!(expected_input_tys.len(), formal_input_tys.len());
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/expectation.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/check/fallback.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/closure.rs" "/checkout/compiler/rustc_typeck/src/check/dropck.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/check/place_op.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
