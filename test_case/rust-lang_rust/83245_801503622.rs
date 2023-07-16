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
Diff in /checkout/library/core/src/slice/specialize.rs at line 1:
-use crate::ptr::write_bytes;
 use crate::mem::{size_of, transmute_copy};
+use crate::ptr::write_bytes;
 
 pub(super) trait SpecFill<T> {
     fn spec_fill(&mut self, value: T);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/slice/sort.rs" "/checkout/library/core/src/slice/ascii.rs" "/checkout/library/core/src/slice/specialize.rs" "/checkout/library/core/src/slice/cmp.rs" "/checkout/library/core/src/slice/iter.rs" "/checkout/library/core/src/slice/mod.rs" "/checkout/library/core/src/slice/rotate.rs" "/checkout/library/core/src/slice/memchr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
