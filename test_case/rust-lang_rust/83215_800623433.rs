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
Diff in /checkout/library/std/src/os/haiku/raw.rs at line 1:
 //! Haiku-specific raw type definitions
 
 #![stable(feature = "raw_ext", since = "1.1.0")]
-#![rustc_deprecated(since = "1.53.0",
-                    reason = "these type aliases are no longer supported by \
+#![rustc_deprecated(
+    since = "1.53.0",
+    reason = "these type aliases are no longer supported by \
                               the standard library, the `libc` crate on \
                               crates.io should be used instead for the correct \
-                              definitions")]
+)]
+)]
 #![allow(deprecated)]
 use crate::os::raw::c_long;
 use crate::os::raw::c_long;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/collections/mod.rs" "/checkout/library/std/src/sys_common/thread_local_dtor.rs" "/checkout/library/std/src/os/freebsd/mod.rs" "/checkout/library/std/src/os/haiku/mod.rs" "/checkout/library/std/src/os/haiku/raw.rs" "/checkout/library/std/src/os/haiku/fs.rs" "/checkout/library/std/src/os/emscripten/mod.rs" "/checkout/library/std/src/f32.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
