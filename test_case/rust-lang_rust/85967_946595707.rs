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
Diff in /checkout/library/std/src/os/unix/mod.rs at line 55:
     pub use crate::os::illumos::*;
     #[cfg(target_os = "ios")]
     pub use crate::os::ios::*;
-    #[cfg(target_os = "linux")]
-    pub use crate::os::linux::*;
     #[cfg(target_os = "l4re")]
     pub use crate::os::l4re::*;
+    #[cfg(target_os = "linux")]
+    pub use crate::os::linux::*;
     #[cfg(target_os = "macos")]
     pub use crate::os::macos::*;
     #[cfg(target_os = "netbsd")]
Diff in /checkout/library/std/src/sys/unix/l4re.rs at line 13:
     use crate::fmt;
     use crate::io::{self, IoSlice, IoSliceMut};
     use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
+    use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
     use crate::sys::fd::FileDesc;
     use crate::sys_common::{AsInner, FromInner, IntoInner};
-    use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
     use crate::time::Duration;
     #[allow(unused_extern_crates)]
     #[allow(unused_extern_crates)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/kernel_copy.rs" "/checkout/library/std/src/os/unix/process.rs" "/checkout/library/std/src/sys/unix/stdio.rs" "/checkout/library/std/src/os/unix/io/fd/tests.rs" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/memchr.rs" "/checkout/library/std/src/sys/unix/rand.rs" "/checkout/library/std/src/os/unix/fs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
