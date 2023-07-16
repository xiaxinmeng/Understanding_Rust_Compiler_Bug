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
Diff in /checkout/library/test/src/formatters/pretty.rs at line 223:
                 self.write_ignored(desc.ignore_message)?;
                 #[cfg(bootstrap)]
                 self.write_ignored(None)?;
+            }
+            }
             TestResult::TrBench(ref bs) => {
                 self.write_bench()?;
                 self.write_plain(&format!(": {}", fmt_bench_samples(bs)))?;
Diff in /checkout/library/test/src/formatters/json.rs at line 132:
                         Some(&*format!(r#""message": "{}""#, EscapedString(msg))),
                 } else {
                 } else {
-                    self.write_event("test", desc.name.as_slice(), "ignored", exec_time, stdout, None)
+                    self.write_event(
+                        "test",
+                        desc.name.as_slice(),
+                        "ignored",
+                        exec_time,
+                        None,
+                    )
                 }
                 }
                 #[cfg(bootstrap)]
                 self.write_event("test", desc.name.as_slice(), "ignored", exec_time, stdout, None)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/test/src/term/terminfo/searcher/tests.rs" "/checkout/library/test/src/formatters/pretty.rs" "/checkout/library/test/src/test_result.rs" "/checkout/library/test/src/time.rs" "/checkout/library/test/src/stats.rs" "/checkout/compiler/rustc_ast/src/ast_like.rs" "/checkout/library/test/src/tests.rs" "/checkout/compiler/rustc_ast/src/expand/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
