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
Diff in /checkout/library/std/src/os/fd/raw.rs at line 5:
 use crate::fs;
 use crate::io;
 use crate::os::raw;
-#[cfg(unix)]
-use crate::os::unix::io::OwnedFd;
 #[cfg(all(doc, unix))]
 use crate::os::unix::io::AsFd;
 #[cfg(all(doc, target_os = "wasi"))]
Diff in /checkout/library/std/src/os/fd/raw.rs at line 13:
 use crate::os::unix::io::AsFd;
+#[cfg(unix)]
+use crate::os::unix::io::OwnedFd;
 #[cfg(target_os = "wasi")]
 use crate::os::wasi::io::OwnedFd;
 use crate::sys_common::{AsInner, IntoInner};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/wasi/io/fd.rs" "/checkout/library/std/src/os/fd/owned.rs" "/checkout/library/std/src/os/fd/raw.rs" "/checkout/library/std/src/os/wasi/io/raw.rs" "/checkout/library/std/src/os/fd/mod.rs" "/checkout/library/std/src/os/wasi/io/mod.rs" "/checkout/library/std/src/os/solid/mod.rs" "/checkout/library/std/src/os/fd/net.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
