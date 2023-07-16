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
Diff in /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs at line 119:
                     && !self.upvars.is_empty()
                 {
                     item_msg = format!("`{}`", access_place_desc.unwrap());
-                    debug_assert!(self.body.local_decls[ty::CAPTURE_STRUCT_LOCAL]
-                        .ty
-                        .is_region_ptr());
+                    debug_assert!(
+                        self.body.local_decls[ty::CAPTURE_STRUCT_LOCAL].ty.is_region_ptr()
+                    );
                     debug_assert!(is_closure_or_generator(
                         Place::ty_from(
                             the_place_err.local,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/var_name.rs" "/checkout/compiler/rustc_mir/src/borrow_check/type_check/mod.rs" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/region_name.rs" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/move_errors.rs" "/checkout/compiler/rustc_mir/src/borrow_check/type_check/liveness/mod.rs" "/checkout/compiler/rustc_mir/src/borrow_check/diagnostics/mod.rs" "/checkout/compiler/rustc_mir/src/borrow_check/type_check/constraint_conversion.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
