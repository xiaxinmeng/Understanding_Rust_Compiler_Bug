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
Diff in /checkout/library/alloc/src/string.rs at line 46:
 use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
 use core::fmt;
 use core::hash;
+use core::iter::FusedIterator;
 #[cfg(not(no_global_oom_handling))]
 use core::iter::{from_fn, FromIterator};
-use core::iter::FusedIterator;
 #[cfg(not(no_global_oom_handling))]
 use core::ops::Add;
 #[cfg(not(no_global_oom_handling))]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/macros.rs" "/checkout/library/alloc/src/rc.rs" "/checkout/library/alloc/src/string.rs" "/checkout/library/alloc/src/sync.rs" "/checkout/library/alloc/src/alloc.rs" "/checkout/library/alloc/src/lib.rs" "/checkout/library/alloc/src/fmt.rs" "/checkout/library/alloc/src/slice.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
