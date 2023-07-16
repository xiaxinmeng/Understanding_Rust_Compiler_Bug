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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 324:
                 args @ [..] => args,
 
 
-            let sugg_tuple_wrap_args = chosen_arg_tys.get(0)
+            let sugg_tuple_wrap_args = chosen_arg_tys
+                .get(0)
                 .cloned()
                 .map(|arg_ty| self.resolve_vars_if_possible(arg_ty))
                 .and_then(|arg_ty| match arg_ty.kind() {
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 331:
                     ty::Tuple(tup_elems) => Some(tup_elems),
                     _ => None,
                 })
-                .map(|tup_elems| {
-                    tup_elems.len() == supplied_arg_count && chosen_arg_tys.len() == 1
-                })
+                .map(|tup_elems| tup_elems.len() == supplied_arg_count && chosen_arg_tys.len() == 1)
                 .and_then(|potential_tuple| {
                     if potential_tuple {
                         match args {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/coercion.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/inherited.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls_overlap.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
