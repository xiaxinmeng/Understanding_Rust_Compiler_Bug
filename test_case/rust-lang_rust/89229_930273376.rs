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
-        region_bound_pairs,
-        borrowck_context,
-        extra
-    ),
+    skip(infcx, body, promoted, region_bound_pairs, borrowck_context, extra),
     level = "debug"
 )]
 fn type_check_internal<'a, 'tcx, R>(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/type_check/mod.rs" "/checkout/compiler/rustc_borrowck/src/type_check/input_output.rs" "/checkout/compiler/rustc_borrowck/src/type_check/constraint_conversion.rs" "/checkout/library/rustc-std-workspace-std/lib.rs" "/checkout/compiler/rustc_borrowck/src/type_check/relate_tys.rs" "/checkout/compiler/rustc_borrowck/src/type_check/free_region_relations.rs" "/checkout/compiler/rustc_borrowck/src/type_check/liveness/polonius.rs" "/checkout/compiler/rustc_borrowck/src/type_check/canonical.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
