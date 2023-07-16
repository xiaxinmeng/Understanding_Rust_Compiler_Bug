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
Diff in /checkout/compiler/rustc_middle/src/traits/select.rs at line 283:
 impl<'tcx> From<OverflowError> for SelectionError<'tcx> {
     fn from(overflow_error: OverflowError) -> SelectionError<'tcx> {
         match overflow_error {
-            OverflowError::Error(e) => {
-                SelectionError::Overflow(OverflowError::Error(e))
-            }
+            OverflowError::Error(e) => SelectionError::Overflow(OverflowError::Error(e)),
             OverflowError::Canonical => SelectionError::Overflow(OverflowError::Canonical),
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/traits/specialization_graph.rs" "/checkout/compiler/rustc_middle/src/traits/mod.rs" "/checkout/compiler/rustc_middle/src/traits/select.rs" "/checkout/compiler/rustc_middle/src/traits/chalk.rs" "/checkout/compiler/rustc_middle/src/ty/consts/valtree.rs" "/checkout/compiler/rustc_middle/src/ty/consts/int.rs" "/checkout/compiler/rustc_middle/src/traits/query.rs" "/checkout/compiler/rustc_middle/src/traits/structural_impls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
