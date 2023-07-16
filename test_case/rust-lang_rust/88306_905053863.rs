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
Diff in /checkout/library/std/src/process/tests.rs at line 133:
             assert_eq!(e.kind(), ErrorKind::NotFound);
             #[cfg(unix)] // Feel free to adjust/disable if this varies on some platform
             assert_eq!(e.to_string(), "No such file or directory (os error 2)");
+        }
+        }
         Ok(..) => panic!(),
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/alloc.rs" "/checkout/library/std/src/backtrace.rs" "/checkout/library/std/src/collections/mod.rs" "/checkout/library/std/src/io/util/tests.rs" "/checkout/library/std/src/io/buffered/linewriter.rs" "/checkout/library/std/src/io/util.rs" "/checkout/library/std/src/io/stdio/tests.rs" "/checkout/library/std/src/process/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
