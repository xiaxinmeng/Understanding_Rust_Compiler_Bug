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
Diff in /checkout/library/std/src/sys/sgx/waitqueue/mod.rs at line 13:
 #[cfg(test)]
 mod tests;
-mod unsafe_list;
 mod spin_mutex;
+mod unsafe_list;
 
 
 use crate::num::NonZeroUsize;
 use crate::ops::{Deref, DerefMut};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/sgx/waitqueue/tests.rs" "/checkout/library/std/src/sys/windows/cmath.rs" "/checkout/library/std/src/sys/sgx/waitqueue/mod.rs" "/checkout/library/std/src/sys/windows/c.rs" "/checkout/library/std/src/sys/sgx/waitqueue/unsafe_list.rs" "/checkout/library/std/src/sys/windows/io.rs" "/checkout/library/std/src/sys/windows/alloc/tests.rs" "/checkout/library/std/src/sys/windows/condvar.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
