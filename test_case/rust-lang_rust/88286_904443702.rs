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
Diff in /checkout/library/std/src/sys/unix/process/process_unix.rs at line 552:
         use crate::os::unix::io::FromRawFd;
         use crate::sys_common::FromInner;
         // Safety: If `pidfd` is nonnegative, we assume it's valid and otherwise unowned.
-        let pidfd = (pidfd >= 0)
-            .then(|| PidFd::from_inner(sys::fd::FileDesc::from_raw_fd(pidfd)));
+        let pidfd = (pidfd >= 0).then(|| PidFd::from_inner(sys::fd::FileDesc::from_raw_fd(pidfd)));
         Process { pid, status: None, pidfd }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_unix.rs" "/checkout/library/std/src/sys/unix/process/process_vxworks.rs" "/checkout/library/std/src/sys/unix/process/process_fuchsia.rs" "/checkout/library/std/src/sys/unix/process/process_unsupported.rs" "/checkout/library/std/src/sys/unix/process/process_common.rs" "/checkout/library/std/src/sys/unix/process/mod.rs" "/checkout/library/std/src/sys/unix/process/zircon.rs" "/checkout/library/std/src/sys/unix/thread.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
