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
Diff in /checkout/library/std/src/env.rs at line 17:
 use crate::ffi::{OsStr, OsString};
 use crate::io;
 use crate::io;
+#[cfg(doc)]
+use crate::path::Path;
 use crate::path::{PathBuf, PathLike};
 use crate::sys;
 use crate::sys::os as os_imp;
Diff in /checkout/library/std/src/env.rs at line 23:
-#[cfg(doc)]
-use crate::path::Path;
 
 /// Returns the current working directory as a [`PathBuf`].
 ///
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/raw_vec/tests.rs" "/checkout/library/std/src/rt.rs" "/checkout/library/alloc/src/boxed/thin.rs" "/checkout/library/std/src/net/display_buffer.rs" "/checkout/library/alloc/src/collections/btree/map/entry.rs" "/checkout/library/alloc/src/collections/btree/set_val.rs" "/checkout/library/alloc/src/collections/btree/map/tests.rs" "/checkout/library/std/src/env.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
