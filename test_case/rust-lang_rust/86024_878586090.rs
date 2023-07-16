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
Diff in /checkout/compiler/rustc_lint/src/expect.rs at line 80:
     sess: &'a Session,
     store: &'a LintStore,
     emitted_lints: Vec<LintIdEmission>,
-    crate_attrs: &'tcx [ast::Attribute]
+    crate_attrs: &'tcx [ast::Attribute],
 
 
 impl<'a, 'tcx> LintExpectationChecker<'a, 'tcx> {
Diff in /checkout/compiler/rustc_lint/src/expect.rs at line 209:
                             lints.extend_from_slice(ids);
                     }
                     }
-                    CheckLintNameResult::Warning(_, _) | CheckLintNameResult::NoLint(_) | CheckLintNameResult::NoTool => {
+                    CheckLintNameResult::Warning(_, _)
+                    | CheckLintNameResult::NoLint(_)
+                    | CheckLintNameResult::NoTool => {
                         // The `LintLevelMapBuilder` will issue a message about this.
                     }
                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/expect.rs" "/checkout/compiler/rustc_lint/src/types.rs" "/checkout/compiler/rustc_lint/src/lib.rs" "/checkout/compiler/rustc_lint/src/nonstandard_style/tests.rs" "/checkout/compiler/rustc_lint/src/unused.rs" "/checkout/compiler/rustc_lint/src/array_into_iter.rs" "/checkout/compiler/rustc_lint/src/non_ascii_idents.rs" "/checkout/compiler/rustc_data_structures/src/sorted_map/index_map.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
