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
Diff in /checkout/library/std/src/os/espidf/raw.rs at line 9:
 )]
 
 
-use crate::os::raw::{c_long};
+use crate::os::raw::c_long;
 use crate::os::unix::raw::{gid_t, uid_t};
 
 #[stable(feature = "pthread_t", since = "1.8.0")]
Build completed unsuccessfully in 0:00:15
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/ios/raw.rs" "/checkout/library/std/src/os/espidf/mod.rs" "/checkout/library/std/src/os/espidf/raw.rs" "/checkout/library/std/src/os/ios/fs.rs" "/checkout/library/std/src/os/espidf/fs.rs" "/checkout/library/std/src/os/ios/mod.rs" "/checkout/library/std/src/sync/once/tests.rs" "/checkout/library/std/src/sync/rwlock.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
