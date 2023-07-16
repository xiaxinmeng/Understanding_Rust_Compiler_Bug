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
Diff in /checkout/compiler/rustc_typeck/src/structured_errors/missing_cast_for_variadic_arg.rs at line 39:
                     format!("{} as {}", snippet, self.cast_ty),
                 )
                 )
-                .help("a function item is zero-sized and needs to be casted \
-                    to a function pointer to be used in FFI")
-                .note("for more information on function items, visit \
-                    https://doc.rust-lang.org/reference/types/function-item.html");
+                .help(
+                    "a function item is zero-sized and needs to be casted \
+                    to a function pointer to be used in FFI",
+                .note(
+                    "for more information on function items, visit \
+                    https://doc.rust-lang.org/reference/types/function-item.html",
+                );
+                );
             } else {
                 err.span_suggestion(
                     self.span,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/structured_errors/missing_cast_for_variadic_arg.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_propagate.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/cfg_build.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior/drop_ranges/record_consumed_borrow.rs" "/checkout/compiler/rustc_typeck/src/constrained_generic_params.rs" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_infer/src/traits/structural_impls.rs" "/checkout/compiler/rustc_typeck/src/structured_errors/wrong_number_of_generic_args.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
