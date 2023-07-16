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
Diff in /checkout/library/std/src/lib.rs at line 524:
 pub mod rt;
 // Platform-abstraction modules
-mod sys_common;
 mod sys;
+mod sys_common;
+mod sys_common;
 
 pub mod alloc;
 
Build completed unsuccessfully in 0:00:14
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/net/ip/tests.rs" "/checkout/library/std/src/lib.rs" "/checkout/library/std/src/sys/windows/io.rs" "/checkout/library/std/src/sys/windows/os.rs" "/checkout/library/std/src/primitive_docs.rs" "/checkout/library/std/src/sys/windows/process/tests.rs" "/checkout/library/std/src/path/tests.rs" "/checkout/library/std/src/time/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
