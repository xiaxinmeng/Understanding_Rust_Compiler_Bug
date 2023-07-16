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
Diff in /checkout/compiler/rustc_typeck/src/check/wfcheck.rs at line 315:
                     )
                 } else {
-                    let mut err = tcx
-                        .sess
-                        .sess
-                        .struct_span_err(
-                            hir_ty.span,
-                            &format!(
-                                "{} is forbidden as the type of a const generic parameter",
-                            ),
-                        );
-                        );
+                    let mut err = tcx.sess.struct_span_err(
+                        hir_ty.span,
+                        &format!(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/regionck.rs" "/checkout/compiler/rustc_typeck/src/check/place_op.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/method/probe.rs" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                            "{} is forbidden as the type of a const generic parameter",
+                        ),
+                    );
+                    );
                     err.note("the only supported types are integers, `bool` and `char`");
                     if tcx.sess.is_nightly_build() {
                         err.help(
Build completed unsuccessfully in 0:00:11
