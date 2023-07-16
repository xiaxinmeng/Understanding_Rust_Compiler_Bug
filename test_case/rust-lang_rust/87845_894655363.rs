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
Diff in /checkout/src/tools/tidy/src/rustdoc_ui_test_parity.rs at line 25:
                     tidy_error!(
                         bad,
                         "{}",
-                        &format!("`{}/{}` is missing from `{}`",
-                            ui_rustdoc_folder,
-                            testname,
-                            rustdoc_ui_folder,
+                        &format!(
+                            "`{}/{}` is missing from `{}`",
+                            ui_rustdoc_folder, testname, rustdoc_ui_folder,
                     );
                 }
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/rustdoc_ui_test_parity.rs" "/checkout/src/librustdoc/passes/unindent_comments/tests.rs" "/checkout/src/tools/tidy/src/error_codes_check.rs" "/checkout/src/tools/tidy/src/unit_tests.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/passes/check_code_block_syntax.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/passes/strip_priv_imports.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
