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
Diff in /checkout/library/test/src/formatters/json.rs at line 138:
                         "ignored",
                         exec_time,
-                        None
+                        None,
                     )
                 }
                 }
                 #[cfg(bootstrap)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/test/src/helpers/exit_code.rs" "/checkout/library/test/src/formatters/mod.rs" "/checkout/library/test/src/formatters/json.rs" "/checkout/library/test/src/formatters/pretty.rs" "/checkout/library/test/src/lib.rs" "/checkout/library/test/src/term/win.rs" "/checkout/library/test/src/helpers/isatty.rs" "/checkout/library/test/src/formatters/junit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
