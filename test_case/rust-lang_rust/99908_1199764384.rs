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
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/build/mod.rs at line 252:
 
             let span_with_body = span_with_body.to(tcx.hir().span(body_id.hir_id));
 
-            build::construct_const(&thir, &infcx, expr, def, id, return_ty, return_ty_span, span_with_body)
+                &thir,
+                &thir,
+                &infcx,
+                def,
+                id,
+                return_ty,
+                return_ty_span,
+                return_ty_span,
+                span_with_body,
+            )
         };
 
         lints::check(tcx, &body);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/scope.rs" "/checkout/compiler/rustc_mir_build/src/build/block.rs" "/checkout/compiler/rustc_mir_build/src/check_unsafety.rs" "/checkout/compiler/rustc_hir/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_constant.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
Build completed unsuccessfully in 0:00:13
