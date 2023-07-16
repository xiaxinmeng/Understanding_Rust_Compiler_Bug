plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 1304:
         // is used and no 'maybe' bounds are used.
         let expanded_traits =
             traits::expand_trait_aliases(tcx, bounds.trait_bounds.iter().map(|&(a, b, _)| (a, b)));
-        let (mut auto_traits, regular_traits): (Vec<_>, Vec<_>) =
-            expanded_traits.partition(|i| {
-                let trait_def = tcx.trait_def(i.trait_ref().def_id());
+        let (mut auto_traits, regular_traits): (Vec<_>, Vec<_>) = expanded_traits.partition(|i| {
+            let trait_def = tcx.trait_def(i.trait_ref().def_id());
 
-                trait_def.has_auto_impl || trait_def.is_marker
-            });
+            trait_def.has_auto_impl || trait_def.is_marker
+        });
         if regular_traits.len() > 1 {
             let first_trait = &regular_traits[0];
             let additional_trait = &regular_traits[1];
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/structured_errors/missing_cast_for_variadic_arg.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs" "/checkout/compiler/rustc_typeck/src/astconv/errors.rs" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs" "/checkout/compiler/rustc_typeck/src/bounds.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/sized_unsized_cast.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
