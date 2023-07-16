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
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 1249:
                 .map(|item| item.ident().ok_or(item.span()))
                 .collect::<Result<Box<[_]>, _>>()
                 .map_err(|span| {
-                    tcx.sess.struct_span_err(span, "must be a name of an associated function").emit();
+                    tcx.sess
+                        .struct_span_err(span, "must be a name of an associated function")
+                        .emit();
                 .ok()
                 .ok()
                 .zip(Some(attr.span)),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/expr_use_visitor.rs" "/checkout/compiler/rustc_typeck/src/collect/type_of.rs" "/checkout/compiler/rustc_typeck/src/collect/item_bounds.rs" "/checkout/compiler/rustc_typeck/src/lib.rs" "/checkout/compiler/rustc_typeck/src/errors.rs" "/checkout/compiler/rustc_typeck/src/variance/solve.rs" "/checkout/compiler/rustc_typeck/src/variance/test.rs" "/checkout/compiler/rustc_typeck/src/collect.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
