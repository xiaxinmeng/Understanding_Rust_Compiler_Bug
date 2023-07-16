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
Diff in /checkout/library/alloc/src/sync/weak.rs at line 3:
 use core::marker::Unsize;
 use core::mem::{self, align_of_val_raw};
 use core::ops::{CoerceUnsized, DispatchFromDyn};
+use core::ptr;
 use core::ptr::NonNull;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/sync/weak.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 use core::sync::atomic;
 use core::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
Diff in /checkout/library/alloc/src/sync/weak.rs at line 9:
-use core::ptr;
 use crate::alloc::{Allocator, Global, Layout};
 use crate::alloc::{Allocator, Global, Layout};
 use crate::rc::is_dangling;
Build completed unsuccessfully in 0:00:15
