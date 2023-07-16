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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/io/error.rs at line 435:
             Unsupported => "unsupported",
             WouldBlock => "operation would block",
             WriteZero => "write zero",
-            InProgress => "in progress"
+            InProgress => "in progress",
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/readbuf.rs" "/checkout/library/std/src/io/tests.rs" "/checkout/library/std/src/io/stdio/tests.rs" "/checkout/library/std/src/io/util.rs" "/checkout/library/std/src/io/impls.rs" "/checkout/library/std/src/io/prelude.rs" "/checkout/library/std/src/io/cursor.rs" "/checkout/library/std/src/io/error.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
