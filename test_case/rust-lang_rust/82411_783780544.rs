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
Diff in /checkout/library/std/src/sys/unix/process/process_unix/tests.rs at line 2:
 fn exitstatus_display_tests() {
     // In practice this is the same on every Unix.
     // If some weird platform turns out to be different, and this test fails, use #[cfg].
-    use crate::process::ExitStatus;
     use crate::os::unix::process::ExitStatusExt;
+    use crate::process::ExitStatus;
 
     let t = |v, s| assert_eq!(s, format!("{}", <ExitStatus as ExitStatusExt>::from_raw(v)));
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_unix.rs" "/checkout/library/std/src/sys/unix/mod.rs" "/checkout/library/std/src/sys/unix/process/process_unix/tests.rs" "/checkout/library/std/src/sys/unix/process/process_common/tests.rs" "/checkout/library/std/src/sys/unix/process/process_common.rs" "/checkout/library/std/src/sys/unix/process/mod.rs" "/checkout/library/std/src/sys/windows/pipe.rs" "/checkout/library/std/src/sys/unix/process/zircon.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
