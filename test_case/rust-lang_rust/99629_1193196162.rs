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
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/move_errors.rs at line 372:
                 match (
                     self.describe_place_with_options(
                         move_place_ref,
-                        DescribePlaceOpt { including_downcast: false, including_tuple_field: false },
+                        DescribePlaceOpt {
+                            including_downcast: false,
+                            including_tuple_field: false,
                     ),
                     ),
                     self.describe_name(move_place_ref),
                     source.describe_for_named_place(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/move_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/outlives_suggestion.rs" "/checkout/compiler/rustc_borrowck/src/dataflow.rs" "/checkout/compiler/rustc_borrowck/src/facts.rs" "/checkout/compiler/rustc_borrowck/src/prefixes.rs" "/checkout/compiler/rustc_borrowck/src/place_ext.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/find_use.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', format.rs:174:19
Build completed unsuccessfully in 0:00:16
