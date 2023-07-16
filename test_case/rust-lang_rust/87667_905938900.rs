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
Diff in /checkout/library/alloc/src/vec/into_iter.rs at line 1:
+use super::AsIntoIter;
 use crate::alloc::{Allocator, Global};
 use crate::raw_vec::RawVec;
 use core::fmt;
Diff in /checkout/library/alloc/src/vec/into_iter.rs at line 9:
 use core::mem::{self};
 use core::ptr::{self, NonNull};
 use core::slice::{self};
-use super::AsIntoIter;
 /// An iterator that moves out of a vector.
 ///
 ///
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/vec/drain_filter.rs" "/checkout/library/alloc/src/vec/into_iter.rs" "/checkout/library/alloc/src/vec/cow.rs" "/checkout/library/alloc/src/vec/in_place_drop.rs" "/checkout/library/alloc/src/vec/spec_from_iter.rs" "/checkout/library/alloc/src/vec/set_len_on_drop.rs" "/checkout/library/alloc/src/vec/splice.rs" "/checkout/library/alloc/src/vec/spec_extend.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
