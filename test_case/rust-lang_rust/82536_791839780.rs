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
Diff in /checkout/compiler/rustc_typeck/src/expr_use_visitor.rs at line 275:
                 if needs_to_be_read {
                     self.borrow_expr(&discr, ty::ImmBorrow);
                 } else {
-                    self.delegate.fake_read(discr_place.place.clone(), FakeReadCause::ForMatchedPlace);
+                    self.delegate
+                        .fake_read(discr_place.place.clone(), FakeReadCause::ForMatchedPlace);
 
                     // We always want to walk the discriminant. We want to make sure, for instance,
                     // that the discriminant has been initialized.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/structured_errors/sized_unsized_cast.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/missing_cast_for_variadic_arg.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsic.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_typeck/src/expr_use_visitor.rs" "/checkout/compiler/rustc_typeck/src/collect/type_of.rs" "/checkout/compiler/rustc_typeck/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
