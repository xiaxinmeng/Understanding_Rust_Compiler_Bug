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
Diff in /checkout/compiler/rustc_typeck/src/check/expr.rs at line 1309:
                     span,
                     E0784,
                     "union expressions should have exactly one field",
-                ).emit();
+                )
+                .emit();
             }
         } else if check_completeness && !error_happened && !remaining_fields.is_empty() {
             let no_accessible_remaining_fields = remaining_fields
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/expr.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs" "/checkout/compiler/rustc_typeck/src/outlives/explicit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
