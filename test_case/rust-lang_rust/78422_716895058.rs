
fmt check
 Diff in /checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs at line 466:
             let abi_of_ty = |ty: Ty<'tcx>| -> Option<&Abi> {
                 match tcx.layout_of(param_env.and(ty)) {
                     Ok(layout) => Some(&layout.abi),
-                    Err(err) => { // #78372
+                    Err(err) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/object_safety.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                        // #78372
