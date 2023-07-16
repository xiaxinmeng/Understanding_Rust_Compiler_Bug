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
Diff in /checkout/library/std/src/num.rs at line 22:
 #[stable(feature = "nonzero", since = "1.28.0")]
 pub use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
 
- #[stable(feature = "int_error_matching", since = "1.54.0")]
+#[stable(feature = "int_error_matching", since = "1.54.0")]
 pub use core::num::IntErrorKind;
 #[cfg(test)]
 #[cfg(test)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/num.rs" "/checkout/library/std/src/f64.rs" "/checkout/library/std/src/sync/condvar/tests.rs" "/checkout/library/std/src/fs.rs" "/checkout/library/std/src/alloc.rs" "/checkout/library/std/src/fs/tests.rs" "/checkout/library/std/src/sys/common/mod.rs" "/checkout/library/std/src/sync/mpsc/mpsc_queue/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
