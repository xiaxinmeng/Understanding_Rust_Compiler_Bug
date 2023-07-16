plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/ty/adt.rs at line 458:
                     Some(Discr { val: b, ty })
                 } else {
                     info!("invalid enum discriminant: {:#?}", val);
-                    tcx.sess.emit_err(crate::error::ConstEvalNonIntError { span: tcx.def_span(expr_did) });
+                    tcx.sess.emit_err(crate::error::ConstEvalNonIntError {
+                        span: tcx.def_span(expr_did),
                     None
                 }
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/closure.rs" "/checkout/compiler/rustc_middle/src/ty/relate.rs" "/checkout/compiler/rustc_middle/src/ty/util.rs" "/checkout/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs" "/checkout/compiler/rustc_middle/src/ty/adt.rs" "/checkout/compiler/rustc_middle/src/ty/generics.rs" "/checkout/compiler/rustc_middle/src/ty/sty.rs" "/checkout/compiler/rustc_middle/src/traits/structural_impls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
