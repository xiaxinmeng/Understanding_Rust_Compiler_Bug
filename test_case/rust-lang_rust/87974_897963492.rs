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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/alloc/tests/slice.rs at line 1:
 use std::cell::Cell;
 use std::cmp::Ordering::{self, Equal, Greater, Less};
 use std::convert::identity;
+use std::fmt;
 use std::mem;
 use std::panic;
 use std::rc::Rc;
Diff in /checkout/library/alloc/tests/slice.rs at line 7:
 use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
-use std::fmt;
 use rand::distributions::Standard;
 use rand::distributions::Standard;
 use rand::seq::SliceRandom;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/tests/binary_heap.rs" "/checkout/library/alloc/tests/slice.rs" "/checkout/library/alloc/tests/borrow.rs" "/checkout/library/alloc/tests/arc.rs" "/checkout/library/alloc/benches/vec_deque.rs" "/checkout/library/alloc/tests/linked_list.rs" "/checkout/library/alloc/benches/str.rs" "/checkout/library/alloc/tests/heap.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
