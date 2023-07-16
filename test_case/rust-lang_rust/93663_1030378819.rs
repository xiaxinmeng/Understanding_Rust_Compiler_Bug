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
Diff in /checkout/library/std/src/os/fd/tests.rs at line 2:
 #[test]
 fn test_raw_fd() {
     #[cfg(unix)]
-    use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd, BorrowedFd};
+    use crate::os::unix::io::{AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
     #[cfg(target_os = "wasi")]
-    use crate::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd, BorrowedFd};
+    use crate::os::wasi::io::{AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
 
     let raw_fd: RawFd = crate::io::stdin().as_raw_fd();
Diff in /checkout/library/std/src/os/fd/tests.rs at line 18:
 #[test]
 fn test_fd() {
 fn test_fd() {
     #[cfg(unix)]
-    use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd, FromRawFd, IntoRawFd, RawFd, AsRawFd};
+    use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
     #[cfg(target_os = "wasi")]
-    use crate::os::wasi::io::{AsFd, BorrowedFd, OwnedFd, FromRawFd, IntoRawFd, RawFd, AsRawFd};
+    use crate::os::wasi::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
 
     let stdin = crate::io::stdin();
     let fd: BorrowedFd<'_> = stdin.as_fd();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/fd/net.rs" "/checkout/library/std/src/os/fd/tests.rs" "/checkout/library/std/src/os/haiku/fs.rs" "/checkout/library/std/src/os/haiku/mod.rs" "/checkout/library/std/src/os/haiku/raw.rs" "/checkout/library/std/src/os/dragonfly/fs.rs" "/checkout/library/std/src/os/dragonfly/mod.rs" "/checkout/library/std/src/os/fd/raw.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
